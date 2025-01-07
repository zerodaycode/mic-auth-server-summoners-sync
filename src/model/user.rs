use serde::Serialize;
use canyon_sql::{date_time::NaiveDateTime, macros::*};

#[derive(CanyonCrud, CanyonMapper, Serialize)]
#[canyon_entity]
pub struct User {
    #[primary_key] pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    #[foreign_key(table = "role", column = "id")]
    pub role: Role // NOTE: for now, a given user can only have ONE role
}

