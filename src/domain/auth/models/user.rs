use canyon_sql::date_time::NaiveDateTime;

use super::role::Role;

/// Represents an authorized and authenticated user within SummonersSync
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub role: Role // NOTE: for now, a given user can only have ONE role
}

