// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::location_type_type::LocationType;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct Location {
    pub id: i64,
    pub description: String,
    pub name: String,
    pub picture: String,
    pub location_type: LocationType,
}

impl __sdk::InModule for Location {
    type Module = super::RemoteModule;
}
