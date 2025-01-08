//! Module which contains all the HTTP REST endpoints related with user authentication and authorization

use rocket::serde::json::Json;

use crate::api_rest::dto::RegisterRequest;

#[post("/sign-up", format = "json", data = "<new_user>")]
pub fn register(new_user: Json<RegisterRequest>) -> String {
    format!("Registering user! Data: {:?}", new_user)
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::rocket;
    use crate::api_rest::controllers::auth::rocket_uri_macro_register;
    use rocket::{http::Status, local::blocking::Client};
    use crate::api_rest::controllers::routes::*;

    const CLIENT: LazyLock<Client> =
        std::sync::LazyLock::new(|| Client::tracked(rocket()).expect("valid rocket instance"));

    #[test]
    fn test_register_user() {
        let expected = "Registering user!";
        let scoped_client = CLIENT; // let binding, due to lifetime issues
        let response = scoped_client.get(String::from(auth::BASE) + &uri!(register).to_string()).dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), expected);
    }
}