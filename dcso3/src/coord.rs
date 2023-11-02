use super::{as_tbl, String};
use crate::{wrapped_table, LuaVec3};
use mlua::{prelude::*, Value};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LLPos {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MGRSPos {
    utm_zone: String,
    mgrs_digraph: String,
    easting: f64,
    northing: f64,
}

impl<'lua> FromLua<'lua> for MGRSPos {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let tbl = as_tbl("MGRSPos", None, value)?;
        Ok(MGRSPos {
            utm_zone: tbl.raw_get("UTMZone")?,
            mgrs_digraph: tbl.raw_get("MGRSDigraph")?,
            easting: tbl.raw_get("Easting")?,
            northing: tbl.raw_get("Northing")?,
        })
    }
}

impl<'lua> IntoLua<'lua> for MGRSPos {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<Value<'lua>> {
        let tbl = lua.create_table()?;
        tbl.raw_set("UTMZone", self.utm_zone)?;
        tbl.raw_set("MGRSDigraph", self.mgrs_digraph)?;
        tbl.raw_set("Easting", self.easting)?;
        tbl.raw_set("Northing", self.northing)?;
        Ok(Value::Table(tbl))
    }
}

wrapped_table!(Coord, None);

impl<'lua> Coord<'lua> {
    pub fn singleton(lua: &'lua Lua) -> LuaResult<Self> {
        lua.globals().raw_get("coord")
    }

    pub fn ll_to_lo(&self, pos: LLPos) -> LuaResult<LuaVec3> {
        self.t
            .call_function("LLtoLO", (pos.latitude, pos.longitude, pos.altitude))
    }

    pub fn lo_to_ll(&self, pos: LuaVec3) -> LuaResult<LLPos> {
        let (latitude, longitude, altitude) = self.t.call_function("LOtoLL", pos)?;
        Ok(LLPos {
            latitude,
            longitude,
            altitude,
        })
    }

    pub fn ll_to_mgrs(&self, latitude: f64, longitude: f64) -> LuaResult<MGRSPos> {
        self.t.call_function("LLtoMGRS", (latitude, longitude))
    }

    pub fn mgrs_to_ll(&self, mgrs: MGRSPos) -> LuaResult<LLPos> {
        let (latitude, longitude, altitude) = self.t.call_function("MGRStoLL", mgrs)?;
        Ok(LLPos {
            latitude,
            longitude,
            altitude,
        })
    }
}