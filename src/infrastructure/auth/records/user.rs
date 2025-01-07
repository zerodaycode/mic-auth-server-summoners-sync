use canyon_sql::{date_time::NaiveDateTime, macros::*};

#[derive(CanyonCrud, CanyonMapper, ForeignKeyable, Debug)]
#[canyon_entity]
pub struct User {
    #[primary_key] pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub role_id: i32
}

