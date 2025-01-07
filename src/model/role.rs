use serde::Serialize;
use canyon_sql::macros::;

#[derive(CanyonCrud, CanyonMapper, Serialize)]
#[canyon_entity]
pub struct Role {
    #[primary_key] pub id: i32,
    pub name: String,
    pub description: String,
}

