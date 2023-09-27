use mlua::{prelude::*, Value};
use serde_derive::Serialize;
use std::ops::{Deref, DerefMut};
pub mod attribute;
pub mod object;
pub mod controller;
pub mod group;
pub mod unit;
pub mod weapon;
pub mod event;
pub mod world;
pub mod airbase;
pub mod warehouse;
pub mod coalition;
pub mod country;
pub mod static_object;

#[macro_export]
macro_rules! wrapped_table {
    ($name:ident, $class:expr) => {
        #[derive(Debug, Clone, Serialize)]
        pub struct $name<'lua> {
            t: mlua::Table<'lua>,
            #[allow(dead_code)]
            #[serde(skip)]
            lua: &'lua Lua
        }

        impl<'lua> Deref for $name<'lua> {
            type Target = mlua::Table<'lua>;

            fn deref(&self) -> &Self::Target {
                &self.t
            }
        }

        impl<'lua> FromLua<'lua> for $name<'lua> {
            fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
                Ok(Self {
                    t: as_tbl(stringify!($name), $class, value)?,
                    lua
                })
            }
        }

        impl<'lua> IntoLua<'lua> for $name<'lua> {
            fn into_lua(self, _lua: &'lua Lua) -> LuaResult<Value<'lua>> {
                Ok(Value::Table(self.t))
            }
        }
    }
}

#[macro_export]
macro_rules! simple_enum {
    ($name:ident, $repr:ident, [$($case:ident => $num:literal),+]) => {
        #[derive(Debug, Clone, Copy, Serialize)]
        #[allow(non_camel_case_types)]
        #[repr($repr)]
        pub enum $name {
            $($case = $num),+
        }
    
        impl<'lua> FromLua<'lua> for $name {
            fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
                Ok(match $repr::from_lua(value, lua)? {
                    $($num => Self::$case),+,
                    _ => return Err(cvt_err(stringify!($name)))
                })
            }
        }
    
        impl<'lua> IntoLua<'lua> for $name {
            fn into_lua(self, _lua: &'lua Lua) -> LuaResult<Value<'lua>> {
                Ok(Value::Integer(self as i64))
            }
        }
    }
}

#[macro_export]
macro_rules! string_enum {
    ($name:ident, $repr:ident, [$($case:ident => $str:literal),+]) => {
        string_enum!($name, $repr, [$($case => $str),+], []);
    };
    ($name:ident, 
     $repr:ident, 
     [$($case:ident => $str:literal),+], 
     [$($altcase:ident => $altstr:literal),*]) => {
        #[derive(Debug, Clone, Serialize)]
        #[allow(non_camel_case_types)]
        #[repr($repr)]
        pub enum $name {
            $($case),+,
            Custom(String)
        }
    
        impl<'lua> FromLua<'lua> for $name {
            fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
                let s = String::from_lua(value, lua)?;
                Ok(match s.as_str() {
                    $($str => Self::$case,)+
                    $($altstr => Self::$altcase,)*
                    _ => Self::Custom(s)
                })
            }
        }
    
        impl<'lua> IntoLua<'lua> for $name {
            fn into_lua(self, lua: &'lua Lua) -> LuaResult<Value<'lua>> {
                Ok(Value::String(match self {
                    $(Self::$case => lua.create_string($str)?),+,
                    Self::Custom(s) => lua.create_string(s.as_str())?
                }))
            }
        }
    };
}

fn cvt_err(to: &'static str) -> LuaError {
    LuaError::FromLuaConversionError {
        from: "value",
        to,
        message: None,
    }
}

fn as_tbl_ref<'a: 'lua, 'lua>(
    to: &'static str,
    value: &'a Value<'lua>,
) -> LuaResult<&'a mlua::Table<'lua>> {
    value.as_table().ok_or_else(|| cvt_err(to))
}

fn check_implements(mut tbl: mlua::Table, class: &str) -> bool {
    loop {
        match tbl.raw_get::<_, String>("className_") {
            Err(_) => break false,
            Ok(s) if s.as_str() == class => break true,
            Ok(_) => match tbl.raw_get::<_, mlua::Table>("parentClass_") {
                Err(_) => break false,
                Ok(t) => {
                    tbl = t;
                }
            },
        }
    }
}

fn as_tbl<'lua>(
    to: &'static str,
    objtyp: Option<&'static str>,
    value: Value<'lua>,
) -> LuaResult<mlua::Table<'lua>> {
    match value {
        Value::Table(tbl) => match objtyp {
            None => Ok(tbl),
            Some(typ) => match tbl.get_metatable() {
                None => Err(LuaError::FromLuaConversionError {
                    from: "table",
                    to: typ,
                    message: Some(format!("table is not an object")),
                }),
                Some(meta) => {
                    if check_implements(meta, typ) {
                        Ok(tbl)
                    } else {
                        Err(LuaError::FromLuaConversionError {
                            from: "table",
                            to: typ,
                            message: Some(format!("object or super expected to have type {}", typ)),
                        })
                    }
                }
            },
        },
        _ => Err(cvt_err(to)),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl<'lua> FromLua<'lua> for Vec2 {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        let tbl = as_tbl("Vec2", None, value)?;
        Ok(Self {
            x: tbl.raw_get("x")?,
            y: tbl.raw_get("y")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl<'lua> FromLua<'lua> for Vec3 {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        let tbl = as_tbl("Vec3", None, value)?;
        Ok(Self {
            x: tbl.raw_get("x")?,
            y: tbl.raw_get("y")?,
            z: tbl.raw_get("z")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Position3 {
    p: Vec3,
    x: Vec3,
    y: Vec3,
    z: Vec3,
}

impl<'lua> FromLua<'lua> for Position3 {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        let tbl = as_tbl("Position3", None, value)?;
        Ok(Self {
            p: tbl.raw_get("p")?,
            x: tbl.raw_get("x")?,
            y: tbl.raw_get("y")?,
            z: tbl.raw_get("z")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Box3 {
    pub min: Vec3,
    pub max: Vec3,
}

impl<'lua> FromLua<'lua> for Box3 {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        let tbl = as_tbl("Box3", None, value)?;
        Ok(Self {
            min: tbl.raw_get("min")?,
            max: tbl.raw_get("max")?,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[repr(transparent)]
pub struct String(compact_str::CompactString);

impl Deref for String {
    type Target = compact_str::CompactString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'lua> IntoLua<'lua> for String {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<Value<'lua>> {
        Ok(Value::String(lua.create_string(self.0)?))
    }
}

impl<'lua> FromLua<'lua> for String {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        use compact_str::{format_compact, CompactString};
        match value {
            Value::String(s) => Ok(Self(CompactString::from(s.to_str()?))),
            Value::Boolean(b) => Ok(Self(format_compact!("{b}"))),
            Value::Integer(n) => Ok(Self(format_compact!("{n}"))),
            Value::Number(n) => Ok(Self(format_compact!("{n}"))),
            v => Ok(Self(CompactString::from(v.to_string()?))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Time(f32);

impl<'lua> FromLua<'lua> for Time {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        Ok(Self(value.as_f32().ok_or_else(|| cvt_err("Time"))?))
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum VolumeType {
    Segment,
    Box,
    Sphere,
    Pyramid,
}
