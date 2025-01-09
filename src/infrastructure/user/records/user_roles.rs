/// A many-to-many relation between the user and it's assigned roles
use canyon_sql::macros::*;

use super::user::User;
use super::role::Role;

#[derive(CanyonCrud, CanyonMapper, Debug)]
#[canyon_entity]
pub struct UserRoles {
    #[foreign_key(table = "user", column = "id")]
    pub user_id: i32,
    #[foreign_key(table = "role", column = "id")]
    pub role_id: i32,
}