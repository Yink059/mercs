/*
Copyright 2024 Eric Stokes.

This file is part of bflib.

bflib is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your
option) any later version.

bflib is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero Public License
for more details.
*/

use std::collections::VecDeque;

use super::{
    objective::{ObjGroupClass, ObjectiveId},
    Db, Set,
};
use crate::{
    cfg::{Crate, Deployable, Troop, UnitTags},
    group, group_by_name, maybe, maybe_mut, objective_mut,
    spawnctx::{Despawn, SpawnCtx, SpawnLoc},
    unit, unit_by_name, unit_mut,
};
use anyhow::{anyhow, bail, Context, Result};
use chrono::prelude::*;
use compact_str::format_compact;
use dcso3::{
    airbase::Airbase,
    atomic_id, azumith3d, centroid2d,
    coalition::Side,
    env::miz::{Group, GroupKind, MizIndex},
    group::GroupCategory,
    land::{Land, SurfaceType},
    net::Ucid,
    object::{DcsObject, DcsOid},
    rotate2d,
    unit::{ClassUnit, Unit},
    LuaVec2, MizLua, Position3, String, Vector2,
};
use enumflags2::BitFlags;
use fxhash::FxHashMap;
use log::{debug, error, warn};
use mlua::{prelude::*, Value};
use serde_derive::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};

atomic_id!(GroupId);
atomic_id!(UnitId);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeployKind {
    Objective,
    Deployed {
        player: Ucid,
        spec: Deployable,
    },
    Troop {
        player: Ucid,
        spec: Troop,
    },
    Crate {
        origin: ObjectiveId,
        player: Ucid,
        spec: Crate,
    },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpawnedUnit {
    pub name: String,
    pub id: UnitId,
    pub group: GroupId,
    pub side: Side,
    pub typ: String,
    pub tags: UnitTags,
    pub template_name: String,
    pub spawn_pos: Vector2,
    pub spawn_heading: f64,
    pub spawn_position: Position3,
    pub pos: Vector2,
    pub heading: f64,
    pub position: Position3,
    pub dead: bool,
    #[serde(skip)]
    pub moved: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedGroup {
    pub id: GroupId,
    pub name: String,
    pub template_name: String,
    pub side: Side,
    pub kind: Option<GroupCategory>,
    pub class: ObjGroupClass,
    pub origin: DeployKind,
    pub units: Set<UnitId>,
    pub tags: UnitTags,
}

impl Db {
    pub fn groups(&self) -> impl Iterator<Item = (&GroupId, &SpawnedGroup)> {
        self.persisted.groups.into_iter()
    }

    pub fn group(&self, id: &GroupId) -> Result<&SpawnedGroup> {
        group!(self, id)
    }

    pub fn group_center(&self, id: &GroupId) -> Result<Vector2> {
        let group = group!(self, id)?;
        Ok(centroid2d(
            group
                .units
                .into_iter()
                .filter_map(|uid| self.persisted.units.get(uid))
                .filter_map(|unit| if unit.dead { None } else { Some(unit.pos) }),
        ))
    }

    pub fn group_by_name(&self, name: &str) -> Result<&SpawnedGroup> {
        group_by_name!(self, name)
    }

    pub fn unit(&self, id: &UnitId) -> Result<&SpawnedUnit> {
        unit!(self, id)
    }

    pub fn unit_by_name(&self, name: &str) -> Result<&SpawnedUnit> {
        unit_by_name!(self, name)
    }

    pub fn first_living_unit(&self, gid: &GroupId) -> Result<&DcsOid<ClassUnit>> {
        group!(self, gid)?
            .units
            .into_iter()
            .find_map(|uid| self.ephemeral.get_object_id_by_uid(uid))
            .ok_or_else(|| anyhow!("all units are dead"))
    }

    pub fn instanced_units(&self) -> impl Iterator<Item = (&SpawnedUnit, &DcsOid<ClassUnit>)> {
        self.persisted
            .units
            .into_iter()
            .filter_map(|(uid, sp)| self.ephemeral.object_id_by_uid.get(uid).map(|id| (sp, id)))
    }

    pub fn deployed(&self) -> impl Iterator<Item = &SpawnedGroup> {
        self.persisted
            .deployed
            .into_iter()
            .chain(self.persisted.troops.into_iter())
            .filter_map(|gid| self.persisted.groups.get(gid))
    }

    pub(super) fn mark_group(&mut self, gid: &GroupId) -> Result<()> {
        if let Some(id) = self.ephemeral.group_marks.remove(gid) {
            self.ephemeral.msgs.delete_mark(id)
        }
        let group = group!(self, gid)?;
        let group_center = centroid2d(
            group
                .units
                .into_iter()
                .map(|uid| self.persisted.units[uid].pos),
        );
        let id = match &group.origin {
            DeployKind::Objective => None,
            DeployKind::Crate { player, spec, .. } => {
                let name = self.persisted.players[player].name.clone();
                let msg = format_compact!("{} {gid} deployed by {name}", spec.name);
                Some(
                    self.ephemeral
                        .msgs
                        .mark_to_side(group.side, group_center, true, msg),
                )
            }
            DeployKind::Deployed { spec, player } => {
                let name = self.persisted.players[player].name.clone();
                let msg = format_compact!("{} {gid} deployed by {name}", spec.path.last().unwrap());
                Some(
                    self.ephemeral
                        .msgs
                        .mark_to_side(group.side, group_center, true, msg),
                )
            }
            DeployKind::Troop { player, spec } => {
                let name = self.persisted.players[player].name.clone();
                let msg = format_compact!("{} {gid} deployed by {name}", spec.name);
                Some(
                    self.ephemeral
                        .msgs
                        .mark_to_side(group.side, group_center, true, msg),
                )
            }
        };
        if let Some(id) = id {
            self.ephemeral.group_marks.insert(*gid, id);
        }
        Ok(())
    }

    pub(super) fn delete_group(&mut self, gid: &GroupId) -> Result<()> {
        let group = self
            .persisted
            .groups
            .remove_cow(gid)
            .ok_or_else(|| anyhow!("no such group {:?}", gid))?;
        self.persisted.groups_by_name.remove_cow(&group.name);
        self.persisted
            .groups_by_side
            .get_mut_cow(&group.side)
            .map(|m| m.remove_cow(gid));
        match &group.origin {
            DeployKind::Objective => (),
            DeployKind::Crate { player, .. } => {
                self.persisted.crates.remove_cow(gid);
                self.persisted.players[player].crates.remove_cow(gid);
            }
            DeployKind::Deployed { spec, .. } => {
                self.persisted.deployed.remove_cow(gid);
                if spec.jtac.is_some() {
                    self.persisted.jtacs.remove_cow(gid);
                }
                if spec.ewr.is_some() {
                    self.persisted.ewrs.remove_cow(gid);
                }
            }
            DeployKind::Troop { spec, .. } => {
                self.persisted.troops.remove_cow(gid);
                if spec.jtac.is_some() {
                    self.persisted.jtacs.remove_cow(gid);
                }
            }
        }
        if let Some(mark) = self.ephemeral.group_marks.remove(gid) {
            self.ephemeral.msgs.delete_mark(mark);
        }
        let mut units: SmallVec<[String; 16]> = smallvec![];
        for uid in &group.units {
            self.ephemeral
                .units_potentially_close_to_enemies
                .remove(uid);
            self.ephemeral.units_potentially_on_walkabout.remove(uid);
            if let Some(id) = self.ephemeral.object_id_by_uid.remove(uid) {
                self.ephemeral.uid_by_object_id.remove(&id);
            }
            if let Some(unit) = self.persisted.units.remove_cow(uid) {
                self.persisted.units_by_name.remove_cow(&unit.name);
                units.push(unit.name);
            }
        }
        self.ephemeral.dirty();
        match group.kind {
            None => {
                // it's a static, we have to get it's units
                for unit in &units {
                    self.ephemeral
                        .push_despawn(*gid, Despawn::Static(unit.clone()))
                }
            }
            Some(_) => {
                // it's a normal group
                self.ephemeral
                    .push_despawn(*gid, Despawn::Group(group.name.clone()));
            }
        }
        Ok(())
    }

    /// add the units to the db, but don't actually spawn them
    pub(super) fn add_group<'lua>(
        &mut self,
        spctx: &'lua SpawnCtx<'lua>,
        idx: &MizIndex,
        side: Side,
        location: SpawnLoc,
        template_name: &str,
        origin: DeployKind,
    ) -> Result<GroupId> {
        fn distance<'a, F: Fn(f64, f64) -> f64>(
            pos: Vector2,
            cmp: F,
            positions: impl IntoIterator<Item = &'a Vector2>,
        ) -> f64 {
            positions
                .into_iter()
                .fold(None, |acc, p| {
                    let d = na::distance_squared(&(*p).into(), &pos.into());
                    let acc = match acc {
                        None => d,
                        Some(d) => d,
                    };
                    Some(cmp(acc, d))
                })
                .map(|d| d.sqrt())
                .unwrap_or(0.)
        }
        fn compute_unit_positions(
            spctx: &SpawnCtx,
            idx: &MizIndex,
            location: SpawnLoc,
            template: &Group,
        ) -> Result<(VecDeque<Vector2>, FxHashMap<String, VecDeque<Vector2>>, f64)> {
            let mut positions = template
                .units()?
                .into_iter()
                .map(|u| Ok(u?.pos()?))
                .collect::<Result<VecDeque<_>>>()?;
            match location {
                SpawnLoc::AtPosWithCenter { pos, center } => {
                    for p in positions.iter_mut() {
                        *p = *p - center + pos;
                    }
                    Ok((positions, FxHashMap::default(), 0.))
                }
                SpawnLoc::AtTrigger {
                    name,
                    group_heading,
                } => {
                    let group_center = centroid2d(positions.iter().map(|p| *p));
                    let pos = spctx.get_trigger_zone(idx, name.as_str())?.pos()?;
                    for p in positions.iter_mut() {
                        *p = *p - group_center + pos;
                    }
                    rotate2d(group_heading, positions.make_contiguous());
                    Ok((positions, FxHashMap::default(), group_heading))
                }
                SpawnLoc::AtPos {
                    pos,
                    offset_direction,
                    group_heading,
                } => {
                    let group_center = centroid2d(positions.iter().map(|p| *p));
                    let radius = distance(group_center, f64::max, positions.iter());
                    for p in positions.iter_mut() {
                        *p = *p - group_center + pos + radius * offset_direction;
                    }
                    rotate2d(group_heading, positions.make_contiguous());
                    let offset_magnitude = 20. - distance(pos, f64::min, positions.iter());
                    for p in positions.iter_mut() {
                        *p = *p + offset_magnitude * offset_direction
                    }
                    Ok((positions, FxHashMap::default(), group_heading))
                }
                SpawnLoc::AtPosWithComponents {
                    pos,
                    group_heading,
                    component_pos,
                } => {
                    let group_center = centroid2d(positions.iter().map(|p| *p));
                    let center_by_typ: FxHashMap<String, Vector2> = {
                        let mut tbl = FxHashMap::default();
                        for unit in template.units()? {
                            let unit = unit?;
                            let pos = unit.pos()?;
                            let typ = unit.typ()?;
                            if component_pos.contains_key(&**typ) {
                                let (n, v) = tbl
                                    .entry(typ.clone())
                                    .or_insert_with(|| (0, Vector2::new(0., 0.)));
                                *v += pos;
                                *n += 1;
                            }
                        }
                        tbl.into_iter()
                            .map(|(k, (n, v))| (k, v / (n as f64)))
                            .collect()
                    };
                    let mut final_position_by_type: FxHashMap<String, VecDeque<Vector2>> =
                        FxHashMap::default();
                    positions.clear();
                    for unit in template.units()? {
                        let unit = unit?;
                        let typ = unit.typ()?;
                        let group_center = match center_by_typ.get(&typ) {
                            None => group_center,
                            Some(pos) => *pos,
                        };
                        match component_pos.get(&typ) {
                            None => positions.push_back(unit.pos()? - group_center + pos),
                            Some(pos) => final_position_by_type
                                .entry(typ.clone())
                                .or_default()
                                .push_back(unit.pos()? - group_center + *pos),
                        }
                    }
                    rotate2d(group_heading, positions.make_contiguous());
                    for positions in final_position_by_type.values_mut() {
                        rotate2d(group_heading, positions.make_contiguous());
                    }
                    Ok((positions, final_position_by_type, group_heading))
                }
            }
        }
        fn check_water(
            land: &Land,
            positions: &VecDeque<Vector2>,
            positions_by_typ: &FxHashMap<String, VecDeque<Vector2>>,
        ) -> Result<()> {
            for pos in positions
                .iter()
                .chain(positions_by_typ.values().flat_map(|v| v.iter()))
            {
                match land.get_surface_type(LuaVec2(*pos))? {
                    SurfaceType::Land | SurfaceType::Road | SurfaceType::Runway => (),
                    SurfaceType::ShallowWater | SurfaceType::Water => {
                        bail!("you can't spawn units in water")
                    }
                }
            }
            Ok(())
        }
        let land = Land::singleton(spctx.lua())?;
        let template_name = String::from(template_name);
        let template = spctx.get_template_ref(idx, GroupKind::Any, side, template_name.as_str())?;
        let (mut positions, mut positions_by_typ, heading) =
            compute_unit_positions(&spctx, idx, location, &template.group)?;
        check_water(&land, &positions, &positions_by_typ)?;
        let kind = GroupCategory::from_kind(template.category);
        let gid = GroupId::new();
        let group_name = String::from(format_compact!("{}-{}", template_name, gid));
        let mut spawned = SpawnedGroup {
            id: gid,
            name: group_name.clone(),
            template_name: template_name.clone(),
            side,
            kind,
            origin,
            class: ObjGroupClass::from(template_name.as_str()),
            units: Set::new(),
            tags: UnitTags(BitFlags::empty()),
        };
        for unit in template.group.units()?.into_iter() {
            let uid = UnitId::new();
            let unit = unit?;
            let typ = unit.typ()?;
            let template_name = unit.name()?;
            let unit_name = String::from(format_compact!("{}-{}", group_name, uid));
            let tags = *self
                .ephemeral
                .cfg
                .unit_classification
                .get(typ.as_str())
                .ok_or_else(|| anyhow!("unit type not classified {typ}"))?;
            spawned.tags.0.insert(tags.0);
            let pos = match positions_by_typ.get_mut(&typ) {
                None => positions.pop_front().unwrap(),
                Some(positions) => positions.pop_front().unwrap(),
            };
            let position = {
                let mut p = Position3::default();
                p.p.x = pos.x;
                p.p.y = land.get_height(LuaVec2(pos))?;
                p.p.z = pos.y;
                p
            };
            let spawned_unit = SpawnedUnit {
                id: uid,
                group: gid,
                side,
                typ,
                tags,
                name: unit_name.clone(),
                template_name,
                spawn_position: position,
                spawn_pos: pos,
                spawn_heading: heading,
                position,
                pos,
                heading,
                dead: false,
                moved: None,
            };
            spawned.units.insert_cow(uid);
            self.persisted.units.insert_cow(uid, spawned_unit);
            self.persisted.units_by_name.insert_cow(unit_name, uid);
        }
        match &mut spawned.origin {
            DeployKind::Objective => (),
            DeployKind::Crate { player, .. } => {
                self.persisted.crates.insert_cow(gid);
                self.persisted.players[player].crates.insert_cow(gid);
            }
            DeployKind::Deployed { spec, .. } => {
                self.persisted.deployed.insert_cow(gid);
                if spec.jtac.is_some() {
                    self.persisted.jtacs.insert_cow(gid);
                }
                if spec.ewr.is_some() {
                    self.persisted.ewrs.insert_cow(gid);
                }
            }
            DeployKind::Troop { spec, .. } => {
                self.persisted.troops.insert_cow(gid);
                if spec.jtac.is_some() {
                    self.persisted.jtacs.insert_cow(gid);
                }
            }
        }
        self.persisted.groups.insert_cow(gid, spawned);
        self.persisted.groups_by_name.insert_cow(group_name, gid);
        self.persisted
            .groups_by_side
            .get_or_default_cow(side)
            .insert_cow(gid);
        self.ephemeral.dirty();
        self.mark_group(&gid)?;
        Ok(gid)
    }

    pub fn add_and_queue_group<'lua>(
        &mut self,
        spctx: &SpawnCtx,
        idx: &MizIndex,
        side: Side,
        location: SpawnLoc,
        template_name: &str,
        origin: DeployKind,
        delay: Option<DateTime<Utc>>,
    ) -> Result<GroupId> {
        let gid = self.add_group(&spctx, idx, side, location, template_name, origin)?;
        match delay {
            None => self.ephemeral.push_spawn(gid),
            Some(at) => self.ephemeral.delayspawnq.entry(at).or_default().push(gid),
        }
        Ok(gid)
    }

    pub fn unit_born(&mut self, lua: MizLua, unit: &Unit) -> Result<()> {
        let id = unit.object_id()?;
        let name = unit.get_name()?;
        if let Some(uid) = self.persisted.units_by_name.get(name.as_str()) {
            self.ephemeral.uid_by_object_id.insert(id.clone(), *uid);
            self.ephemeral.object_id_by_uid.insert(*uid, id.clone());
            self.ephemeral
                .units_potentially_close_to_enemies
                .insert(*uid);
            self.ephemeral.units_potentially_on_walkabout.insert(*uid);
        }
        let slot = unit.slot()?;
        if let Some(oid) = self.persisted.objectives_by_slot.get(&slot) {
            self.ephemeral
                .slot_by_object_id
                .insert(id.clone(), slot.clone());
            self.ephemeral.object_id_by_slot.insert(slot.clone(), id);
            let obj = objective_mut!(self, oid)?;
            let mut adjust_warehouse = || -> Result<()> {
                let sifo = maybe!(obj.slots, slot, "slot")?;
                let id = maybe!(self.ephemeral.airbase_by_oid, obj.id, "airbase")?;
                let wh = Airbase::get_instance(lua, id)
                    .context("getting airbase")?
                    .get_warehouse()
                    .context("getting warehouse")?;
                if sifo.ground_start {
                    wh.remove_item(sifo.typ.0.clone(), 1).with_context(|| {
                        format_compact!("removing {} from warehouse", sifo.typ.0)
                    })?;
                    for wep in unit.get_ammo()? {
                        let wep = wep?;
                        let count = wep.count()?;
                        let typ = wep.type_name()?;
                        let whcnt = wh.get_item_count(typ.clone())?;
                        debug!("removing {count} {typ} from the warehouse which contains {whcnt}");
                        wh.remove_item(typ.clone(), count)?;
                        if let Some(inv) = obj.warehouse.equipment.get_mut_cow(&typ) {
                            inv.stored = whcnt - count;
                        }
                    }
                }
                maybe_mut!(obj.warehouse.equipment, sifo.typ.0, "equip")?.stored =
                    wh.get_item_count(sifo.typ.0.clone()).with_context(|| {
                        format_compact!("getting warehouse count for {}", sifo.typ.0)
                    })?;
                Ok(())
            };
            if let Err(e) = adjust_warehouse() {
                error!("couldn't adjust warehouse {:?}", e)
            }
            self.player_entered_unit(unit)
                .context("entering player into unit")?;
            self.ephemeral.dirty()
        }
        Ok(())
    }

    pub fn unit_dead(
        &mut self,
        lua: MizLua,
        id: &DcsOid<ClassUnit>,
        now: DateTime<Utc>,
    ) -> Result<()> {
        let uid = match self.ephemeral.unit_dead(id) {
            None => return Ok(()),
            Some((uid, ucid)) => {
                if let Some(ucid) = ucid {
                    let player = &mut self.persisted.players[&ucid];
                    if let Some((_, Some(inst))) = player.current_slot.take() {
                        if let Some(oid) = inst.landed_at_objective {
                            if let Err(e) = self.sync_vehicle_at_obj(lua, oid, inst.typ.clone()) {
                                error!(
                                    "failed to sync warehouse at {:?} for vehicle {:?} {:?}",
                                    oid, inst.typ, e
                                )
                            }
                        }
                    }
                }
                uid
            }
        };
        match self.persisted.units.get_mut_cow(&uid) {
            None => error!("unit_dead: missing unit {:?}", uid),
            Some(unit) => {
                unit.dead = true;
                unit.pos = unit.spawn_pos;
                unit.heading = unit.spawn_heading;
                unit.position = unit.spawn_position;
                self.ephemeral.dirty();
                let gid = unit.group;
                if let Some(oid) = self.persisted.objectives_by_group.get(&gid).copied() {
                    self.update_objective_status(&oid, now)?
                }
                if self.persisted.deployed.contains(&gid)
                    || self.persisted.troops.contains(&gid)
                    || self.persisted.crates.contains(&gid)
                {
                    if self.group_health(&gid)?.0 == 0 {
                        self.delete_group(&gid)?
                    }
                }
            }
        }
        Ok(())
    }

    pub fn group_health(&self, gid: &GroupId) -> Result<(usize, usize)> {
        let group = group!(self, gid)?;
        let mut alive = 0;
        for uid in &group.units {
            if !unit!(self, uid)?.dead {
                alive += 1;
            }
        }
        Ok((alive, group.units.len()))
    }

    pub fn update_unit_positions<'a, I: Iterator<Item = UnitId> + 'a>(
        &'a mut self,
        lua: MizLua,
        units: Option<I>,
    ) -> Result<Vec<DcsOid<ClassUnit>>> {
        let mut unit: Option<Unit> = None;
        let mut moved: SmallVec<[GroupId; 16]> = smallvec![];
        let mut dead: Vec<DcsOid<ClassUnit>> = vec![];
        let units = units
            .map(|i| Box::new(i) as Box<dyn Iterator<Item = UnitId>>)
            .unwrap_or_else(|| {
                Box::new(self.ephemeral.units_able_to_move.iter().map(|i| *i))
                    as Box<dyn Iterator<Item = UnitId>>
            });
        for uid in units {
            let id = match self.ephemeral.object_id_by_uid.get(&uid) {
                Some(id) => id,
                None => {
                    warn!("update_unit_positions skipping unknown unit {uid}");
                    continue;
                }
            };
            let instance = match unit.take() {
                Some(unit) => unit.change_instance(id),
                None => Unit::get_instance(lua, id),
            };
            let instance = match instance {
                Ok(i) => i,
                Err(e) => {
                    warn!(
                        "update_unit_positions skipping invalid instance {uid}, {:?}",
                        e
                    );
                    dead.push(id.clone());
                    continue;
                }
            };
            let pos = instance.get_position()?;
            let point = Vector2::new(pos.p.x, pos.p.z);
            let heading = azumith3d(pos.x.0);
            let spunit = unit_mut!(self, uid)?;
            if spunit.position != pos {
                moved.push(spunit.group);
                spunit.position = pos;
                spunit.pos = point;
                spunit.heading = heading;
                self.ephemeral
                    .units_potentially_close_to_enemies
                    .insert(uid);
                self.ephemeral.units_potentially_on_walkabout.insert(uid);
            }
            unit = Some(instance);
        }
        for gid in moved {
            self.ephemeral.dirty();
            self.mark_group(&gid)?;
        }
        Ok(dead)
    }
}
