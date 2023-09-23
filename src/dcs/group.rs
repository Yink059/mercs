use super::{as_tbl, coalition::Side, controller::Controller, cvt_err, unit::Unit, String};
use mlua::{prelude::*, Value};
use serde_derive::Serialize;

#[derive(Debug, Clone, Serialize)]
#[repr(u8)]
pub enum GroupCategory {
    Airplane = 0,
    Ground = 1,
    Helicopter = 2,
    Ship = 3,
    Train = 4,
}

impl<'lua> IntoLua<'lua> for GroupCategory {
    fn into_lua(self, _: &'lua Lua) -> LuaResult<Value<'lua>> {
        Ok(Value::Integer(self as i64))
    }
}

impl<'lua> FromLua<'lua> for GroupCategory {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        Ok(match u32::from_lua(value, lua)? {
            0 => GroupCategory::Airplane,
            1 => GroupCategory::Ground,
            2 => GroupCategory::Helicopter,
            3 => GroupCategory::Ship,
            4 => GroupCategory::Train,
            _ => return Err(cvt_err("GroupCategory")),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Owner {
    Contested,
    Side(Side),
}

impl<'lua> FromLua<'lua> for Owner {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match u32::from_lua(value.clone(), lua)? {
            3 => Ok(Self::Contested),
            _ => Ok(Owner::Side(Side::from_lua(value, lua)?)),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Group<'lua> {
    t: mlua::Table<'lua>,
    #[serde(skip)]
    lua: &'lua Lua,
}

impl<'lua> FromLua<'lua> for Group<'lua> {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        Ok(Self {
            t: as_tbl("Group", Some("Group"), value)?,
            lua,
        })
    }
}

impl<'lua> IntoLua<'lua> for Group<'lua> {
    fn into_lua(self, _: &'lua Lua) -> LuaResult<Value<'lua>> {
        Ok(Value::Table(self.t.clone()))
    }
}

impl<'lua> Group<'lua> {
    pub fn get_by_name(lua: &'lua Lua, name: &str) -> LuaResult<Group<'lua>> {
        let globals = lua.globals();
        let unit = as_tbl("Group", Some("Group"), globals.raw_get("Group")?)?;
        Self::from_lua(unit.call_method("getByName", name)?, lua)
    }

    pub fn destroy(&self) -> LuaResult<()> {
        self.t.call_method("destroy", ())
    }

    pub fn activate(&self) -> LuaResult<()> {
        self.t.call_method("activate", ())
    }

    pub fn get_category(&self) -> LuaResult<GroupCategory> {
        self.t.call_method("getCategory", ())
    }

    pub fn get_coalition(&self) -> LuaResult<Owner> {
        self.t.call_method("getCoalition", ())
    }

    pub fn get_name(&self) -> LuaResult<String> {
        self.t.call_method("getName", ())
    }

    pub fn get_id(&self) -> LuaResult<u32> {
        self.t.call_method("getID", ())
    }

    pub fn get_size(&self) -> LuaResult<u32> {
        self.t.call_method("getSize", ())
    }

    pub fn get_initial_size(&self) -> LuaResult<u32> {
        self.t.call_method("getInitialSize", ())
    }

    pub fn get_unit(&self, index: usize) -> LuaResult<Unit> {
        Unit::from_lua(self.t.call_method("getUnit", index)?, self.lua)
    }

    pub fn get_units(&self) -> LuaResult<impl Iterator<Item = LuaResult<Unit>>> {
        Ok(as_tbl("Units", None, self.t.call_method("getUnits", ())?)?.sequence_values())
    }

    pub fn get_controller(&self) -> LuaResult<Controller> {
        Ok(Controller::from_lua(
            self.t.call_method("getController", ())?,
            self.lua,
        )?)
    }

    pub fn enable_emission(&self, on: bool) -> LuaResult<()> {
        self.t.call_method("enableEmission", on)
    }
}