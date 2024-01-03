use crate::{
    cfg::{Cfg, LimitEnforceTyp, UnitTag},
    db::{
        self,
        cargo::{Cargo, Oldest, SlotStats},
        Db,
    },
    ewr::EwrUnits,
    Context,
};
use anyhow::{anyhow, bail, Result};
use chrono::prelude::*;
use compact_str::{format_compact, CompactString};
use dcso3::{
    as_tbl,
    coalition::Side,
    env::miz::{Group, GroupId, Miz},
    lua_err,
    mission_commands::{GroupSubMenu, MissionCommands},
    net::SlotId,
    MizLua, String,
};
use enumflags2::{BitFlag, BitFlags};
use fxhash::FxHashMap;
use log::debug;
use mlua::{prelude::*, Value};
use std::collections::hash_map::Entry;

#[derive(Debug)]
struct ArgTuple<T, U> {
    fst: T,
    snd: U,
}

impl<'lua, T, U> IntoLua<'lua> for ArgTuple<T, U>
where
    T: IntoLua<'lua>,
    U: IntoLua<'lua>,
{
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let tbl = lua.create_table()?;
        tbl.raw_set("fst", self.fst)?;
        tbl.raw_set("snd", self.snd)?;
        Ok(Value::Table(tbl))
    }
}

impl<'lua, T, U> FromLua<'lua> for ArgTuple<T, U>
where
    T: FromLua<'lua>,
    U: FromLua<'lua>,
{
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let tbl = as_tbl("ArgTuple", None, value).map_err(lua_err)?;
        Ok(Self {
            fst: tbl.raw_get("fst")?,
            snd: tbl.raw_get("snd")?,
        })
    }
}

fn slot_for_group(lua: MizLua, ctx: &Context, gid: &GroupId) -> Result<(Side, SlotId)> {
    let miz = Miz::singleton(lua)?;
    let group = miz
        .get_group(&ctx.idx, gid)?
        .ok_or_else(|| anyhow!("no such group {:?}", gid))?;
    let units = group.group.units()?;
    if units.len() > 1 {
        bail!(
            "groups with more than one member can't spawn crates {:?}",
            gid
        )
    }
    let unit = units.first()?;
    Ok((group.side, unit.slot()?))
}

fn player_name(db: &Db, slot: &SlotId) -> String {
    db.player_in_slot(&slot)
        .and_then(|ucid| db.player(ucid).map(|p| p.name.clone()))
        .unwrap_or_default()
}

fn unpakistan(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (side, slot) = slot_for_group(lua, ctx, &gid)?;
    match ctx.db.unpakistan(lua, &ctx.idx, &slot) {
        Ok(unpakistan) => {
            let player = player_name(&ctx.db, &slot);
            let msg = format_compact!("{player} {unpakistan}");
            ctx.db.msgs().panel_to_side(10, false, side, msg);
        }
        Err(e) => {
            let msg = format_compact!("{}", e);
            ctx.db.msgs().panel_to_group(10, false, gid, msg)
        }
    }
    Ok(())
}

fn load_crate(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (side, slot) = slot_for_group(lua, ctx, &gid)?;
    match ctx.db.load_nearby_crate(lua, &ctx.idx, &slot) {
        Ok(cr) => {
            let (dep_name, dep) = ctx
                .db
                .deployable_by_crate(&side, &cr.name)
                .ok_or_else(|| anyhow!("unknown deployable for crate {}", cr.name))?;
            let (n, oldest) = ctx.db.number_deployed(side, dep_name.as_str())?;
            let enforce = match dep.limit_enforce {
                LimitEnforceTyp::DenyCrate => {
                    format_compact!("unpacking will be denied when the limit is exceeded")
                }
                LimitEnforceTyp::DeleteOldest => match oldest {
                    Some(Oldest::Group(gid)) => {
                        format_compact!(
                            "unpacking will delete oldest, {}, when the limit is exceeded",
                            gid
                        )
                    }
                    Some(Oldest::Objective(oid)) => {
                        format_compact!(
                            "unpacking will delete oldest, {}, when the limit is exceeded",
                            oid
                        )
                    }
                    None => {
                        format_compact!("unpacking will delete oldest when the limit is exceeded")
                    }
                },
            };
            let msg = format_compact!(
                "{} crate loaded, {n}/{} deployed, {}",
                cr.name,
                dep.limit,
                enforce
            );
            ctx.db.msgs().panel_to_group(10, false, gid, msg)
        }
        Err(e) => {
            let msg = format_compact!("crate could not be loaded: {}", e);
            ctx.db.msgs().panel_to_group(10, false, gid, msg)
        }
    }
    Ok(())
}

fn unload_crate(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (_side, slot) = slot_for_group(lua, ctx, &gid)?;
    match ctx.db.unload_crate(lua, &ctx.idx, &slot) {
        Ok(cr) => {
            let msg = format_compact!("{} crate unloaded", cr.name);
            ctx.db.msgs().panel_to_group(10, false, gid, msg)
        }
        Err(e) => {
            let msg = format_compact!("{}", e);
            ctx.db.msgs().panel_to_group(10, false, gid, msg)
        }
    }
    Ok(())
}

pub(super) fn list_cargo_for_slot(lua: MizLua, ctx: &mut Context, slot: &SlotId) -> Result<()> {
    let cargo = Cargo::default();
    let cargo = ctx.db.list_cargo(&slot).unwrap_or(&cargo);
    let uinfo = ctx.db.slot_miz_unit(lua, &ctx.idx, &slot)?;
    let capacity = ctx.db.cargo_capacity(&uinfo.unit)?;
    let mut msg = CompactString::new("Current Cargo\n----------------------------\n");
    msg.push_str(&format_compact!(
        "troops: {} of {}\n",
        cargo.num_troops(),
        capacity.troop_slots
    ));
    msg.push_str(&format_compact!(
        "crates: {} of {}\n",
        cargo.num_crates(),
        capacity.crate_slots
    ));
    msg.push_str(&format_compact!(
        "total : {} of {}\n",
        cargo.num_total(),
        capacity.total_slots
    ));
    msg.push_str("----------------------------\n");
    let mut total = 0;
    for (_, cr) in &cargo.crates {
        msg.push_str(&format_compact!(
            "{} crate weighing {} kg\n",
            cr.name,
            cr.weight
        ));
        total += cr.weight
    }
    for tr in &cargo.troops {
        msg.push_str(&format_compact!(
            "{} troop weiging {} kg\n",
            tr.name,
            tr.weight
        ));
        total += tr.weight
    }
    if total > 0 {
        msg.push_str("----------------------------\n");
    }
    msg.push_str(&format_compact!("total cargo weight: {} kg", total as u32));
    ctx.db
        .msgs()
        .panel_to_unit(15, false, slot.as_unit_id().unwrap(), msg);
    Ok(())
}

pub fn list_current_cargo(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (_side, slot) = slot_for_group(lua, ctx, &gid)?;
    list_cargo_for_slot(lua, ctx, &slot)
}

fn list_nearby_crates(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (_side, slot) = slot_for_group(lua, ctx, &gid)?;
    let st = SlotStats::get(&ctx.db, lua, &slot)?;
    let nearby = ctx.db.list_nearby_crates(&st)?;
    if nearby.len() > 0 {
        let mut msg = CompactString::new("");
        for nc in nearby {
            msg.push_str(&format_compact!(
                "{} crate, bearing {}, {} meters away\n",
                nc.crate_def.name,
                nc.heading as u32,
                nc.distance as u32
            ));
        }
        ctx.db.msgs().panel_to_group(10, false, gid, msg)
    } else {
        drop(nearby);
        ctx.db
            .msgs()
            .panel_to_group(10, false, gid, "No nearby crates")
    }
    Ok(())
}

fn destroy_nearby_crate(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (_side, slot) = slot_for_group(lua, ctx, &gid)?;
    if let Err(e) = ctx.db.destroy_nearby_crate(lua, &slot) {
        ctx.db
            .msgs()
            .panel_to_group(10, false, gid, format_compact!("{}", e))
    }
    Ok(())
}

fn spawn_crate(lua: MizLua, arg: ArgTuple<GroupId, String>) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (_side, slot) = slot_for_group(lua, ctx, &arg.fst)?;
    match ctx.db.spawn_crate(lua, &ctx.idx, &slot, &arg.snd) {
        Err(e) => ctx
            .db
            .msgs()
            .panel_to_group(10, false, arg.fst, format_compact!("{e}")),
        Ok(st) => {
            if let Some(max_crates) = ctx.db.cfg().max_crates {
                let (n, oldest) = ctx.db.number_crates_deployed(&st)?;
                let msg = match oldest {
                    None => format_compact!("{n}/{max_crates} crates spawned"),
                    Some(gid) => format_compact!(
                        "{n}/{max_crates} crates spawned, {gid} will be deleted if the limit is exceeded"
                    ),
                };
                ctx.db.msgs().panel_to_group(10, false, arg.fst, msg)
            }
        }
    }
    Ok(())
}

fn load_troops(lua: MizLua, arg: ArgTuple<GroupId, String>) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (side, slot) = slot_for_group(lua, ctx, &arg.fst)?;
    match ctx.db.load_troops(lua, &ctx.idx, &slot, &arg.snd) {
        Ok(tr) => {
            let (n, oldest) = ctx.db.number_troops_deployed(side, &tr.name)?;
            let player = player_name(&ctx.db, &slot);
            let enforce = match tr.limit_enforce {
                LimitEnforceTyp::DenyCrate => {
                    format_compact!("unloading will be denied when the limit is exceeded")
                }
                LimitEnforceTyp::DeleteOldest => match oldest {
                    Some(gid) => {
                        format_compact!(
                            "unloading will delete oldest, {gid}, when the limit is exceeded"
                        )
                    }
                    None => {
                        format_compact!("unloading will delete oldest when the limit is exceeded")
                    }
                },
            };
            let msg = format_compact!("{player} loaded {}, {n}/{}, {}", tr.name, tr.limit, enforce);
            ctx.db.msgs().panel_to_side(10, false, side, msg)
        }
        Err(e) => ctx
            .db
            .msgs()
            .panel_to_group(10, false, arg.fst, format_compact!("{e}")),
    }
    Ok(())
}

fn unload_troops(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (side, slot) = slot_for_group(lua, ctx, &gid)?;
    match ctx.db.unload_troops(lua, &ctx.idx, &slot) {
        Ok(tr) => {
            let player = player_name(&ctx.db, &slot);
            let msg = format_compact!("{player} dropped {} troops into the field", tr.name);
            ctx.db.msgs().panel_to_side(10, false, side, msg)
        }
        Err(e) => ctx
            .db
            .msgs()
            .panel_to_group(10, false, gid, format_compact!("{e}")),
    }
    Ok(())
}

fn extract_troops(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (side, slot) = slot_for_group(lua, ctx, &gid)?;
    match ctx.db.extract_troops(lua, &ctx.idx, &slot) {
        Ok(tr) => {
            let player = player_name(&ctx.db, &slot);
            let msg = format_compact!("{player} extracted {} troops from the field", tr.name);
            ctx.db.msgs().panel_to_side(10, false, side, msg)
        }
        Err(e) => ctx
            .db
            .msgs()
            .panel_to_group(10, false, gid, format_compact!("{e}")),
    }
    Ok(())
}

fn return_troops(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (side, slot) = slot_for_group(lua, ctx, &gid)?;
    match ctx.db.return_troops(lua, &ctx.idx, &slot) {
        Ok(tr) => {
            let player = player_name(&ctx.db, &slot);
            let msg = format_compact!("{player} returned {} troops", tr.name);
            ctx.db.msgs().panel_to_side(10, false, side, msg)
        }
        Err(e) => ctx
            .db
            .msgs()
            .panel_to_group(10, false, gid, format_compact!("{e}")),
    }
    Ok(())
}

fn toggle_ewr(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (_, slot) = slot_for_group(lua, ctx, &gid)?;
    if let Some(ucid) = ctx.db.player_in_slot(&slot) {
        let st = if ctx.ewr.toggle(ucid) {
            "enabled"
        } else {
            "disabled"
        };
        ctx.db
            .msgs()
            .panel_to_group(5, false, gid, format_compact!("ewr reports are {st}"))
    }
    Ok(())
}

fn ewr_report(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (_, slot) = slot_for_group(lua, ctx, &gid)?;
    let mut report = format_compact!("Bandits BRAA\n");
    if let Some(ucid) = ctx.db.player_in_slot(&slot) {
        if let Some(player) = ctx.db.player(ucid) {
            if let Some((_, Some(inst))) = &player.current_slot {
                let chickens = ctx.ewr.where_chicken(Utc::now(), false, ucid, player, inst);
                for braa in chickens {
                    report.push_str(&format_compact!("{braa}\n"));
                }
            }
        }
    }
    ctx.db.msgs().panel_to_group(10, false, gid, report);
    Ok(())
}

fn friendly_ewr_report(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (_, slot) = slot_for_group(lua, ctx, &gid)?;
    let mut report = format_compact!("Friendlies Nearby\n");
    if let Some(ucid) = ctx.db.player_in_slot(&slot) {
        if let Some(player) = ctx.db.player(ucid) {
            if let Some((_, Some(inst))) = &player.current_slot {
                let friendlies = ctx.ewr.where_chicken(Utc::now(), true, ucid, player, inst);
                for braa in friendlies {
                    report.push_str(&format_compact!("{braa}\n"));
                }
            }
        }
    }
    ctx.db.msgs().panel_to_group(10, false, gid, report);
    Ok(())
}

fn ewr_units_imperial(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (_, slot) = slot_for_group(lua, ctx, &gid)?;
    if let Some(ucid) = ctx.db.player_in_slot(&slot) {
        ctx.ewr.set_units(ucid, EwrUnits::Imperial);
        ctx.db
            .msgs()
            .panel_to_group(5, false, gid, "EWR units are now Imperial");
    }
    Ok(())
}

fn ewr_units_metric(lua: MizLua, gid: GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let (_, slot) = slot_for_group(lua, ctx, &gid)?;
    if let Some(ucid) = ctx.db.player_in_slot(&slot) {
        ctx.ewr.set_units(ucid, EwrUnits::Imperial);
        ctx.db
            .msgs()
            .panel_to_group(5, false, gid, "EWR units are now Metric");
    }
    Ok(())
}

fn add_troops_menu_for_group(
    cfg: &Cfg,
    mc: &MissionCommands,
    side: &Side,
    group: GroupId,
) -> Result<()> {
    if let Some(squads) = cfg.troops.get(side) {
        let root = mc.add_submenu_for_group(group, "Troops".into(), None)?;
        mc.add_command_for_group(
            group,
            "Unload".into(),
            Some(root.clone()),
            unload_troops,
            group,
        )?;
        mc.add_command_for_group(
            group,
            "Extract".into(),
            Some(root.clone()),
            extract_troops,
            group,
        )?;
        mc.add_command_for_group(
            group,
            "List".into(),
            Some(root.clone()),
            list_current_cargo,
            group,
        )?;
        mc.add_command_for_group(
            group,
            "Return".into(),
            Some(root.clone()),
            return_troops,
            group,
        )?;
        let root = mc.add_submenu_for_group(group, "Squads".into(), Some(root))?;
        for sq in squads {
            mc.add_command_for_group(
                group,
                format_compact!("Load {} squad", sq.name).into(),
                Some(root.clone()),
                load_troops,
                ArgTuple {
                    fst: group,
                    snd: sq.name.clone(),
                },
            )?;
        }
    }
    Ok(())
}

fn add_cargo_menu_for_group(
    cfg: &Cfg,
    mc: &MissionCommands,
    side: &Side,
    group: GroupId,
) -> Result<()> {
    let root = mc.add_submenu_for_group(group, "Cargo".into(), None)?;
    mc.add_command_for_group(
        group,
        "Unpakistan!".into(),
        Some(root.clone()),
        unpakistan,
        group,
    )?;
    mc.add_command_for_group(
        group,
        "Load Nearby Crate".into(),
        Some(root.clone()),
        load_crate,
        group,
    )?;
    mc.add_command_for_group(
        group,
        "Unload Crate".into(),
        Some(root.clone()),
        unload_crate,
        group,
    )?;
    mc.add_command_for_group(
        group,
        "List Nearby Crates".into(),
        Some(root.clone()),
        list_nearby_crates,
        group,
    )?;
    mc.add_command_for_group(
        group,
        "List Cargo".into(),
        Some(root.clone()),
        list_current_cargo,
        group,
    )?;
    mc.add_command_for_group(
        group,
        "Destroy Nearby Crate".into(),
        Some(root.clone()),
        destroy_nearby_crate,
        group,
    )?;
    let root = mc.add_submenu_for_group(group, "Crates".into(), Some(root.clone()))?;
    let rep = &cfg.repair_crate[side];
    mc.add_command_for_group(
        group,
        rep.name.clone(),
        Some(root.clone()),
        spawn_crate,
        ArgTuple {
            fst: group,
            snd: rep.name.clone(),
        },
    )?;
    let mut created_menus: FxHashMap<String, GroupSubMenu> = FxHashMap::default();
    for dep in cfg.deployables.get(side).unwrap_or(&vec![]) {
        let root = dep
            .path
            .iter()
            .fold(Ok(root.clone()), |root: Result<_>, p| {
                let root = root?;
                match created_menus.entry(p.clone()) {
                    Entry::Occupied(e) => Ok(e.get().clone()),
                    Entry::Vacant(e) => Ok(e
                        .insert(mc.add_submenu_for_group(group, p.clone(), Some(root))?)
                        .clone()),
                }
            })?;
        for cr in dep.crates.iter().chain(dep.repair_crate.iter()) {
            let title = if cr.required > 1 {
                String::from(format_compact!("{}({})", cr.name, cr.required))
            } else {
                cr.name.clone()
            };
            mc.add_command_for_group(
                group,
                title,
                Some(root.clone()),
                spawn_crate,
                ArgTuple {
                    fst: group,
                    snd: cr.name.clone(),
                },
            )?;
        }
    }
    Ok(())
}

fn add_ewr_menu_for_group(mc: &MissionCommands, group: GroupId) -> Result<()> {
    let root = mc.add_submenu_for_group(group, "Where Chicken?".into(), None)?;
    mc.add_command_for_group(
        group,
        "Gib BRAA!".into(),
        Some(root.clone()),
        ewr_report,
        group,
    )?;
    mc.add_command_for_group(
        group,
        "toggle".into(),
        Some(root.clone()),
        toggle_ewr,
        group,
    )?;
    mc.add_command_for_group(
        group,
        "Friendly Report".into(),
        Some(root.clone()),
        friendly_ewr_report,
        group,
    )?;
    mc.add_command_for_group(
        group,
        "Units to Imperial".into(),
        Some(root.clone()),
        ewr_units_imperial,
        group,
    )?;
    mc.add_command_for_group(
        group,
        "Units to Metric".into(),
        Some(root.clone()),
        ewr_units_metric,
        group,
    )?;
    Ok(())
}

fn jtac_status(_: MizLua, gid: db::GroupId) -> Result<()> {
    let ctx = unsafe { Context::get_mut() };
    let side = ctx.db.group(&gid)?.side;
    let msg = ctx.jtac.jtac_status(&ctx.db, &gid)?;
    ctx.db.msgs().panel_to_side(10, false, side, msg);
    Ok(())
}

fn jtac_toggle_auto_laser(lua: MizLua, gid: db::GroupId) -> Result<()> {
    {
        let ctx = unsafe { Context::get_mut() };
        ctx.jtac.toggle_auto_laser(lua, &gid)?;
    }
    jtac_status(lua, gid)
}

fn jtac_toggle_smoke_target(lua: MizLua, gid: db::GroupId) -> Result<()> {
    {
        let ctx = unsafe { Context::get_mut() };
        ctx.jtac.toggle_smoke_target(&gid)?;
    }
    jtac_status(lua, gid)
}

fn jtac_shift(lua: MizLua, gid: db::GroupId) -> Result<()> {
    {
        let ctx = unsafe { Context::get_mut() };
        ctx.jtac.shift(lua, &gid)?;
    }
    jtac_status(lua, gid)
}

fn jtac_clear_filter(lua: MizLua, gid: db::GroupId) -> Result<()> {
    {
        let ctx = unsafe { Context::get_mut() };
        ctx.jtac.clear_filter(lua, &gid)?;
    }
    jtac_status(lua, gid)
}

fn jtac_filter(lua: MizLua, arg: ArgTuple<db::GroupId, u32>) -> Result<()> {
    {
        let ctx = unsafe { Context::get_mut() };
        let filter =
            BitFlags::<UnitTag>::from_bits(arg.snd).map_err(|_| anyhow!("invalid filter bits"))?;
        for tag in filter.iter() {
            ctx.jtac.add_filter(lua, &arg.fst, tag)?;
        }
    }
    jtac_status(lua, arg.fst)
}

fn jtac_set_code(lua: MizLua, arg: ArgTuple<db::GroupId, u16>) -> Result<()> {
    {
        let ctx = unsafe { Context::get_mut() };
        ctx.jtac.set_code_part(lua, &arg.fst, arg.snd)?;
    }
    jtac_status(lua, arg.fst)
}

pub fn add_menu_for_jtac(lua: MizLua, side: Side, group: db::GroupId) -> Result<()> {
    let mc = MissionCommands::singleton(lua)?;
    let root = mc.add_submenu_for_coalition(side, "JTAC".into(), None)?;
    let root =
        mc.add_submenu_for_coalition(side, format_compact!("{:?}", group).into(), Some(root))?;
    mc.add_command_for_coalition(
        side,
        "Status".into(),
        Some(root.clone()),
        jtac_status,
        group,
    )?;
    mc.add_command_for_coalition(
        side,
        "Toggle Auto Laser".into(),
        Some(root.clone()),
        jtac_toggle_auto_laser,
        group,
    )?;
    mc.add_command_for_coalition(
        side,
        "Toggle Smoke Target".into(),
        Some(root.clone()),
        jtac_toggle_smoke_target,
        group,
    )?;
    mc.add_command_for_coalition(side, "Shift".into(), Some(root.clone()), jtac_shift, group)?;
    let filter_root = mc.add_submenu_for_coalition(side, "Filter".into(), Some(root.clone()))?;
    mc.add_command_for_coalition(
        side,
        "Clear".into(),
        Some(filter_root.clone()),
        jtac_clear_filter,
        group,
    )?;
    for tag in UnitTag::all().iter() {
        mc.add_command_for_coalition(
            side,
            format_compact!("{:?}", tag).into(),
            Some(filter_root.clone()),
            jtac_filter,
            ArgTuple {
                fst: group,
                snd: BitFlags::from(tag).bits(),
            },
        )?;
    }
    let code_root = mc.add_submenu_for_coalition(side, "Code".into(), Some(root.clone()))?;
    let hundreds_root =
        mc.add_submenu_for_coalition(side, "Hundreds".into(), Some(code_root.clone()))?;
    let tens_root = mc.add_submenu_for_coalition(side, "Tens".into(), Some(code_root.clone()))?;
    let ones_root = mc.add_submenu_for_coalition(side, "Ones".into(), Some(code_root.clone()))?;
    for (scale, root) in [
        (100, &hundreds_root),
        (10, &tens_root),
        (1, &ones_root),
    ] {
        let range = if scale == 100 {
            0..=6
        } else {
            0..=8
        };
        for n in range {
            mc.add_command_for_coalition(
                side,
                format_compact!("{n}").into(),
                Some(root.clone()),
                jtac_set_code,
                ArgTuple {
                    fst: group,
                    snd: n * scale,
                },
            )?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, Default)]
struct CarryCap {
    troops: bool,
    crates: bool,
}

impl CarryCap {
    fn new(cfg: &Cfg, group: &Group) -> Result<CarryCap> {
        Ok(group
            .units()?
            .into_iter()
            .fold(Ok(Self::default()), |acc: Result<Self>, unit| {
                let mut acc = acc?;
                let unit = unit?;
                let typ = unit.typ()?;
                match cfg.cargo.get(&**typ) {
                    None => Ok(acc),
                    Some(c) => {
                        acc.troops |= c.troop_slots > 0 && c.total_slots > 0;
                        acc.crates |= c.crate_slots > 0 && c.total_slots > 0;
                        Ok(acc)
                    }
                }
            })?)
    }
}

pub(super) fn init(ctx: &Context, lua: MizLua) -> Result<()> {
    debug!("initializing menus");
    let cfg = ctx.db.cfg();
    let miz = Miz::singleton(lua)?;
    let mc = MissionCommands::singleton(lua)?;
    for side in [Side::Red, Side::Blue, Side::Neutral] {
        let coa = miz.coalition(side)?;
        for country in coa.countries()? {
            let country = country?;
            for heli in country.helicopters()? {
                let heli = heli?;
                let cap = CarryCap::new(cfg, &heli)?;
                let gid = heli.id()?;
                if cap.crates {
                    add_cargo_menu_for_group(cfg, &mc, &side, gid)?
                }
                if cap.troops {
                    add_troops_menu_for_group(cfg, &mc, &side, gid)?
                }
                add_ewr_menu_for_group(&mc, gid)?;
            }
            for plane in country.planes()? {
                let plane = plane?;
                let cap = CarryCap::new(cfg, &plane)?;
                let gid = plane.id()?;
                if cap.crates {
                    add_cargo_menu_for_group(cfg, &mc, &side, gid)?
                }
                if cap.troops {
                    add_troops_menu_for_group(cfg, &mc, &side, gid)?
                }
                add_ewr_menu_for_group(&mc, gid)?;
            }
        }
        let _ = mc.add_submenu_for_coalition(side, "JTAC".into(), None)?;
    }
    Ok(())
}
