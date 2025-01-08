use serde::Deserialize;

/// The `DTO` that helds the incoming data for a new user to be registered
#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String
}
