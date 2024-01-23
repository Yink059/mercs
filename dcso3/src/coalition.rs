/*
Copyright 2024 Eric Stokes.

This file is part of dcso3.

dcso3 is free software: you can redistribute it and/or modify it under
the terms of the MIT License.

dcso3 is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE.
*/

use super::{
    airbase::Airbase,
    as_tbl,
    country::Country,
    cvt_err, env,
    group::{Group, GroupCategory},
    static_object::StaticObject,
    unit::Unit,
};
use crate::{simple_enum, wrapped_table, LuaEnv, MizLua, Sequence};
use anyhow::{anyhow, bail, Result};
use mlua::{prelude::*, Value};
use serde_derive::{Deserialize, Serialize};
use std::{fmt, ops::Deref, str::FromStr};

simple_enum!(Side, u8, [Neutral => 0, Red => 1, Blue => 2, Green => 3, Merc1 => 4, Merc2 => 5 , Merc3 => 6]);
pub const SIDES: [Side; 7] = [Side::Neutral, Side::Red, Side::Blue, Side::Green, Side::Merc1, Side::Merc2, Side::Merc3];

impl Default for Side {
    fn default() -> Self {
        Side::Red
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl FromStr for Side {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "blue" => Side::Blue,
            "red" => Side::Red,
            "neutrals" => Side::Neutral,
            s => bail!("unknown side {s}"),
        })
    }
}

impl Side {
    pub fn to_str(&self) -> &'static str {
        match self {
            Side::Blue => "blue",
            Side::Red => "red",
            Side::Neutral => "neutrals",
            Side::Green => "green",
            Side::Merc1 => "merc1",
            Side::Merc2 => "merc2",
            Side::Merc3 => "merc3"
        }
    }

    pub fn opposite(&self) -> Side {
        match self {
            Self::Blue => Self::Red,
            Self::Red => Self::Blue,
            Self::Neutral => Self::Neutral,
            Self::Green => Self::Neutral,
            Self::Merc1 => Self::Green,
            Self::Merc2 => Self::Green,
            Self::Merc3 => Self::Green
        }
    }
}

#[derive(Debug, Clone)]
pub enum Static<'lua> {
    Airbase(Airbase<'lua>),
    Static(StaticObject<'lua>),
}

simple_enum!(Service, u8, [Atc => 0, Awacs => 1, Fac => 3, Tanker => 2]);
wrapped_table!(Coalition, None);

impl<'lua> Coalition<'lua> {
    pub fn singleton(lua: MizLua<'lua>) -> Result<Self> {
        Ok(Self {
            t: lua.inner().globals().raw_get("coalition")?,
            lua: lua.inner(),
        })
    }

    pub fn add_group(
        &self,
        country: Country,
        category: GroupCategory,
        data: env::miz::Group<'lua>,
    ) -> Result<Group<'lua>> {
        Ok(self
            .t
            .call_function("addGroup", (country, category, data))?)
    }

    pub fn add_static_object(
        &self,
        country: Country,
        data: env::miz::Unit<'lua>,
    ) -> Result<Static<'lua>> {
        let tbl: LuaTable = self.t.call_function("addStaticObject", (country, data))?;
        let mt = tbl
            .get_metatable()
            .ok_or_else(|| anyhow!("returned static object has no meta table"))?;
        if mt.raw_get::<_, String>("className_")?.as_str() == "Airbase" {
            Ok(Static::Airbase(Airbase::from_lua(
                Value::Table(tbl),
                self.lua,
            )?))
        } else {
            Ok(Static::Static(StaticObject::from_lua(
                Value::Table(tbl),
                self.lua,
            )?))
        }
    }

    pub fn get_groups(&self, side: Side) -> Result<Sequence<'lua, Group<'lua>>> {
        Ok(self.t.call_function("getGroups", side)?)
    }

    pub fn get_static_objects(&self, side: Side) -> Result<Sequence<'lua, StaticObject<'lua>>> {
        Ok(self.t.call_function("getStaticObjects", side)?)
    }

    pub fn get_airbases(&self, side: Side) -> Result<Sequence<'lua, Airbase<'lua>>> {
        Ok(self.t.call_function("getAirbases", side)?)
    }

    pub fn get_players(&self, side: Side) -> Result<Sequence<'lua, Unit<'lua>>> {
        Ok(self.t.call_function("getPlayers", side)?)
    }

    pub fn get_service_providers(
        &self,
        side: Side,
        service: Service,
    ) -> Result<Sequence<'lua, Unit<'lua>>> {
        Ok(self
            .t
            .call_function("getServiceProviders", (side, service))?)
    }

    pub fn get_country_coalition(&self, country: Country) -> Result<Side> {
        Ok(self.t.call_function("getCountrySide", country)?)
    }
}
