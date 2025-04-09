use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::generated::Hero;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct SqlHero {
    id: i64,
    pub level: i32,
    pub name: String,
    #[sqlx(rename = "othername")]
    // #[serde(skip_serializing_if = "String::is_empty")]
    pub other_name: Option<String>,
    pub picture: String,
    pub powers: String,
}

impl From<SqlHero> for Hero {
    fn from(value: SqlHero) -> Self {
        Self {
            space_id: 0,
            id: value.id,
            level: value.level,
            name: value.name,
            other_name: value.other_name,
            picture: value.picture,
            powers: value.powers,
        }
    }
}
