extern crate nalgebra as na;
use compact_str::format_compact;
use dcso3::{
    coalition::{Coalition, Side},
    env::miz::{GroupInfo, GroupKind, Miz, MizIndex, TriggerZone},
    err,
    group::GroupCategory,
    DeepClone, String, Vector2,
};
use fxhash::FxHashMap;
use immutable_chunkmap::{map::MapM as Map, set::SetM as Set};
use mlua::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{self, File},
    path::{Path, PathBuf}, sync::atomic::{AtomicU64, Ordering},
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct GroupId(u64);

static MAX_GROUP_ID: AtomicU64 = AtomicU64::new(0);

impl Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for GroupId {
    fn default() -> Self {
        GroupId(0)
    }
}

impl GroupId {
    pub fn new() -> Self {
        Self(MAX_GROUP_ID.fetch_add(1, Ordering::Relaxed))
    }

    fn update_max(id: Self) {
        // not strictly thread safe, but it doesn't matter in this context
        if id.0 >= MAX_GROUP_ID.load(Ordering::Relaxed) {
            MAX_GROUP_ID.store(id.0 + 1, Ordering::Relaxed)
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct UnitId(u64);

static MAX_UNIT_ID: AtomicU64 = AtomicU64::new(0);

impl Default for UnitId {
    fn default() -> Self {
        UnitId(0)
    }
}

impl Display for UnitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UnitId {
    pub fn new() -> Self {
        Self(MAX_UNIT_ID.fetch_add(1, Ordering::Relaxed))
    }

    fn update_max(id: Self) {
        // not strictly thread safe, but it doesn't matter in this context
        if id.0 >= MAX_UNIT_ID.load(Ordering::Relaxed) {
            MAX_UNIT_ID.store(id.0 + 1, Ordering::Relaxed)
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpawnedUnit {
    pub name: String,
    pub id: UnitId,
    pub group: GroupId,
    pub template_name: String,
    pub pos: Vector2,
    pub dead: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedGroup {
    pub id: GroupId,
    pub name: String,
    pub template_name: String,
    pub side: Side,
    pub kind: GroupKind,
    pub units: Set<UnitId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnLoc {
    AtPos(Vector2),
    AtTrigger { name: String, offset: Vector2 },
}

pub struct SpawnCtx<'lua> {
    coalition: Coalition<'lua>,
    miz: Miz<'lua>,
    lua: &'lua Lua,
}

impl<'lua> SpawnCtx<'lua> {
    pub fn new(lua: &'lua Lua) -> LuaResult<Self> {
        Ok(Self {
            coalition: Coalition::singleton(lua)?,
            miz: Miz::singleton(lua)?,
            lua,
        })
    }

    pub fn get_template(
        &self,
        idx: &MizIndex,
        kind: GroupKind,
        side: Side,
        template_name: &str,
    ) -> LuaResult<GroupInfo> {
        let mut template = self
            .miz
            .get_group(idx, kind, side, template_name)?
            .ok_or_else(|| err("no such template"))?;
        template.group = template.group.deep_clone(self.lua)?;
        Ok(template)
    }

    pub fn get_trigger_zone(&self, idx: &MizIndex, name: &str) -> LuaResult<TriggerZone> {
        Ok(self
            .miz
            .get_trigger_zone(idx, name)?
            .ok_or_else(|| err("no such trigger zone"))?)
    }

    pub fn spawn(&self, template: GroupInfo) -> LuaResult<()> {
        match GroupCategory::from_kind(template.category) {
            None => self
                .coalition
                .add_static_object(template.country, template.group),
            Some(category) => self
                .coalition
                .add_group(template.country, category, template.group),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Db {
    #[serde(skip)]
    dirty: bool,
    groups_by_id: Map<GroupId, SpawnedGroup>,
    units_by_id: Map<UnitId, SpawnedUnit>,
    groups_by_name: Map<String, GroupId>,
    units_by_name: Map<String, UnitId>,
    groups_by_side: Map<Side, Set<GroupId>>,
}

impl Db {
    pub fn load(path: &Path) -> LuaResult<Self> {
        let file = File::open(&path).map_err(|e| {
            println!("failed to open save file {:?}, {:?}", path, e);
            err("io error")
        })?;
        let db: Self = serde_json::from_reader(file).map_err(|e| {
            println!("failed to decode save file {:?}, {:?}", path, e);
            err("decode error")
        })?;
        for (id, _) in &db.groups_by_id {
            GroupId::update_max(*id)
        }
        for (id, _) in &db.units_by_id {
            UnitId::update_max(*id)
        }
        Ok(db)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let mut tmp = PathBuf::from(path);
        tmp.set_extension("tmp");
        let file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&tmp)?;
        serde_json::to_writer(file, self)?;
        fs::rename(tmp, path)?;
        Ok(())
    }

    pub fn maybe_snapshot(&mut self) -> Option<Self> {
        if self.dirty {
            self.dirty = false;
            Some(self.clone())
        } else {
            None
        }
    }

    pub fn unit_dead(&mut self, id: UnitId, dead: bool) {
        self.units_by_id.update_cow(id, (), |id, (), unit| {
            unit.map(|(_, unit)| {
                let unit = SpawnedUnit {
                    dead,
                    ..unit.clone()
                };
                (id, unit)
            })
        });
        self.dirty = true;
    }

    pub fn groups(&self) -> impl Iterator<Item = (&GroupId, &SpawnedGroup)> {
        self.groups_by_id.into_iter()
    }

    pub fn get_group(&self, id: &GroupId) -> Option<&SpawnedGroup> {
        self.groups_by_id.get(id)
    }

    pub fn get_group_by_name(&self, name: &str) -> Option<&SpawnedGroup> {
        self.groups_by_name.get(name).and_then(|gid| self.groups_by_id.get(gid))
    }

    pub fn get_unit(&self, id: &UnitId) -> Option<&SpawnedUnit> {
        self.units_by_id.get(id)
    }

    pub fn get_unit_by_name(&self, name: &str) -> Option<&SpawnedUnit> {
        self.units_by_name.get(name).and_then(|uid| self.get_unit(uid))
    }

    pub fn respawn_group<'lua>(
        &self,
        idx: &MizIndex,
        spctx: &SpawnCtx,
        group: &SpawnedGroup,
    ) -> LuaResult<()> {
        let template =
            spctx.get_template(idx, group.kind, group.side, group.template_name.as_str())?;
        template.group.set("lateActivation", false)?;
        template.group.set_name(group.name.clone())?;
        let by_tname: FxHashMap<&str, &SpawnedUnit> = group
            .units
            .into_iter()
            .filter_map(|uid| {
                self.units_by_id.get(uid).and_then(|u| {
                    if u.dead {
                        None
                    } else {
                        Some((u.template_name.as_str(), u))
                    }
                })
            })
            .collect();
        let alive = {
            let units = template.group.units()?;
            let mut i = 1;
            while i as usize <= units.len() {
                let unit = units.get(i)?;
                match by_tname.get(unit.name()?.as_str()) {
                    None => units.remove(i)?,
                    Some(su) => {
                        template.group.set_pos(su.pos)?;
                        unit.set_pos(su.pos)?;
                        i += 1;
                    }
                }
            }
            units.len() > 0
        };
        if alive {
            spctx.spawn(template)
        } else {
            Ok(())
        }
    }

    pub fn spawn_template_as_new<'lua>(
        &mut self,
        lua: &'lua Lua,
        idx: &MizIndex,
        side: Side,
        kind: GroupKind,
        location: &SpawnLoc,
        template_name: &str,
    ) -> LuaResult<GroupId> {
        let spctx = SpawnCtx::new(lua)?;
        let template_name = String::from(template_name);
        let template = spctx.get_template(idx, kind, side, template_name.as_str())?;
        let pos = match location {
            SpawnLoc::AtPos(pos) => *pos,
            SpawnLoc::AtTrigger { name, offset } => {
                spctx.get_trigger_zone(idx, name.as_str())?.pos()? + offset
            }
        };
        let gid = GroupId::new();
        let group_name = String::from(format_compact!("{}-{}", template_name, gid));
        template.group.set("lateActivation", false)?;
        template.group.raw_remove("groupId")?;
        let orig_group_pos = template.group.pos()?;
        template.group.set_pos(pos)?;
        template.group.set_name(group_name.clone())?;
        let mut spawned = SpawnedGroup {
            id: gid,
            name: group_name.clone(),
            template_name: template_name.clone(),
            side,
            kind,
            units: Set::new(),
        };
        for unit in template.group.units()? {
            let uid = UnitId::new();
            let unit = unit?;
            let template_name = unit.name()?;
            let unit_name = String::from(format_compact!("{}-{}", group_name, uid));
            let unit_pos_offset = orig_group_pos - unit.pos()?;
            let pos = pos + unit_pos_offset;
            unit.raw_remove("unitId")?;
            unit.set_pos(pos)?;
            unit.set_name(unit_name.clone())?;
            let spawned_unit = SpawnedUnit {
                id: uid,
                group: gid,
                name: unit_name.clone(),
                template_name,
                pos,
                dead: false,
            };
            spawned.units.insert_cow(uid);
            self.units_by_id.insert_cow(uid, spawned_unit);
            self.units_by_name.insert_cow(unit_name, uid);
        }
        self.groups_by_id.insert_cow(gid, spawned);
        self.groups_by_name.insert_cow(group_name, gid);
        self.dirty = true;
        spctx.spawn(template)?;
        Ok(gid)
    }
}