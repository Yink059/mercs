pub mod db;
extern crate nalgebra as na;
use compact_str::format_compact;
use db::{Db, GroupId, SpawnLoc, UnitId};
use dcso3::{
    coalition::Side,
    env::{
        self,
        miz::{GroupKind, Miz},
        Env,
    },
    err,
    event::Event,
    lfs::Lfs,
    timer::Timer,
    world::World,
    wrap_unit, String, UserHooks, Vector2,
};
use fxhash::FxHashMap;
use mlua::prelude::*;
use std::{path::PathBuf, sync::mpsc, thread};

#[derive(Debug)]
enum BgTask {
    MizInit,
    SaveState(PathBuf, Db),
}

fn background_loop(rx: mpsc::Receiver<BgTask>) {
    while let Ok(msg) = rx.recv() {
        match msg {
            BgTask::MizInit => (),
            BgTask::SaveState(path, db) => match db.save(&path) {
                Ok(()) => (),
                Err(e) => println!("failed to save state to {:?}, {:?}", path, e),
            },
        }
    }
}

#[derive(Debug, Default)]
struct Context {
    idx: env::miz::MizIndex,
    db: Db,
    to_background: Option<mpsc::Sender<BgTask>>,
    units_by_obj_id: FxHashMap<i64, UnitId>,
}

static mut CONTEXT: Option<Context> = None;

impl Context {
    // this must be used cautiously. Reasons why it's not totally nuts,
    // - the dcs scripting api is single threaded
    // - the event handlers can be triggerred by api calls, making refcells and mutexes error prone
    // - as long as an event handler doesn't step on state in an api call it's ok, since concurrency never happens
    //   that isn't so hard to guarantee
    fn get_mut() -> &'static mut Context {
        unsafe {
            match CONTEXT.as_mut() {
                Some(ctx) => ctx,
                None => {
                    println!("init ctx");
                    CONTEXT = Some(Context::default());
                    CONTEXT.as_mut().unwrap()
                }
            }
        }
    }

    fn get() -> &'static Context {
        Context::get_mut()
    }

    fn do_background_task(&mut self, task: BgTask) {
        if self.to_background.is_none() {
            let (tx, rx) = mpsc::channel();
            self.to_background = Some(tx);
            thread::spawn(move || background_loop(rx));
        }
        match self.to_background.as_ref().unwrap().send(task) {
            Ok(()) => (),
            Err(_) => println!("background loop died"),
        }
    }

    fn spawn_template_as_new(
        &mut self,
        lua: &Lua,
        side: Side,
        kind: GroupKind,
        location: &SpawnLoc,
        template_name: &str,
    ) -> LuaResult<GroupId> {
        self.db
            .spawn_template_as_new(lua, &self.idx, side, kind, location, template_name)
    }

    fn respawn_groups(&mut self, lua: &Lua) -> LuaResult<()> {
        let spctx = db::SpawnCtx::new(lua)?;
        for (_, group) in self.db.groups() {
            self.db.respawn_group(&self.idx, &spctx, group)?
        }
        Ok(())
    }
}

fn on_player_try_connect(
    _: &Lua,
    addr: String,
    name: String,
    ucid: String,
    id: u32,
) -> LuaResult<bool> {
    println!(
        "onPlayerTryConnect addr: {:?}, name: {:?}, ucid: {:?}, id: {:?}",
        addr, name, ucid, id
    );
    Ok(true)
}

fn on_player_try_send_chat(_: &Lua, id: u32, msg: String, all: bool) -> LuaResult<String> {
    println!(
        "onPlayerTrySendChat id: {:?}, msg: {:?}, all: {:?}",
        id, msg, all
    );
    Ok(msg)
}

fn on_player_try_change_slot(_: &Lua, id: u32, side: Side, slot: String) -> LuaResult<bool> {
    println!(
        "onPlayerTryChangeSlot id: {:?}, side: {:?}, slot: {:?}",
        id, side, slot
    );
    Ok(true)
}

fn on_event(_lua: &Lua, ev: Event) -> LuaResult<()> {
    println!("onEventTranslated: {:?}", ev);
    let ctx = Context::get_mut();
    match ev {
        Event::Birth(b) => {
            if let Ok(unit) = b.initiator.as_unit() {
                let name = unit.as_object()?.get_name()?;
                if let Some(su) = ctx.db.get_unit_by_name(name.as_str()) {
                    let uid = su.id;
                    let oid: i64 = unit.get_object_id()?;
                    ctx.units_by_obj_id.insert(oid, uid);
                }
            }
        }
        Event::Dead(e) => {
            if let Ok(unit) = e.initiator.as_unit() {
                let id = unit.get_object_id()?;
                if let Some(uid) = ctx.units_by_obj_id.remove(&id) {
                    ctx.db.unit_dead(uid, true);
                }
            }
        }
        _ => (),
    }
    Ok(())
}

fn on_mission_load_end(lua: &Lua) -> LuaResult<()> {
    println!("on_mission_load_end");
    let miz = env::miz::Miz::singleton(lua)?;
    println!("indexing mission");
    let ctx = Context::get_mut();
    ctx.idx = miz.index()?;
    ctx.do_background_task(BgTask::MizInit);
    println!("indexed mission");
    Ok(())
}

fn on_simulation_start(_lua: &Lua) -> LuaResult<()> {
    println!("on_simulation_start");
    Ok(())
}

fn init_hooks_(lua: &Lua) -> LuaResult<()> {
    println!("setting user hooks");
    UserHooks::new(lua)
        .on_simulation_start(on_simulation_start)?
        .on_mission_load_end(on_mission_load_end)?
        .on_player_try_change_slot(on_player_try_change_slot)?
        .on_player_try_connect(on_player_try_connect)?
        .on_player_try_send_chat(on_player_try_send_chat)?
        .register()?;
    println!("set user hooks");
    Ok(())
}

fn init_hooks(lua: &Lua, _: ()) -> LuaResult<()> {
    wrap_unit("init_hooks", init_hooks_(lua))
}

fn spawn_new(lua: &Lua, ctx: &mut Context) -> LuaResult<()> {
    ctx.spawn_template_as_new(
        lua,
        Side::Blue,
        GroupKind::Vehicle,
        &SpawnLoc::AtTrigger {
            name: "TEST_TZ".into(),
            offset: Vector2::new(100., 100.),
        },
        "BLUE_TEST_GROUP",
    )?;
    ctx.spawn_template_as_new(
        lua,
        Side::Red,
        GroupKind::Vehicle,
        &SpawnLoc::AtTrigger {
            name: "TEST_TZ".into(),
            offset: Vector2::new(-100., -100.),
        },
        "RED_TEST_GROUP",
    )?;
    Ok(())
}

fn init_miz_(lua: &Lua) -> LuaResult<()> {
    let ctx = Context::get_mut();
    println!("adding event handler");
    World::get(lua)?.add_event_handler(on_event)?;
    let sortie = Miz::singleton(lua)?.sortie()?;
    let path = match Env::singleton(lua)?.get_value_dict_by_key(sortie)?.as_str() {
        "" => return Err(err("missing sortie in miz file")),
        s => PathBuf::from(format_compact!("{}\\{}", Lfs::singleton(lua)?.writedir()?, s).as_str()),
    };
    let timer = Timer::singleton(lua)?;
    timer.schedule_function(timer.get_time()? + 10., mlua::Value::Nil, {
        let path = path.clone();
        move |_lua, _, now| {
            let ctx = Context::get_mut();
            if let Some(snap) = ctx.db.maybe_snapshot() {
                ctx.do_background_task(BgTask::SaveState(path.clone(), snap));
            }
            Ok(Some(now + 10.))
        }
    })?;
    println!("spawning");
    if !path.exists() {
        spawn_new(lua, ctx)?;
    } else {
        ctx.db = Db::load(&path)?;
        ctx.respawn_groups(lua)?
    }
    println!("spawned");
    Ok(())
}

fn init_miz(lua: &Lua, _: ()) -> LuaResult<()> {
    wrap_unit("init_miz", init_miz_(lua))
}

#[mlua::lua_module]
fn bflib(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("initHooks", lua.create_function(init_hooks)?)?;
    exports.set("initMiz", lua.create_function(init_miz)?)?;
    Ok(exports)
}
