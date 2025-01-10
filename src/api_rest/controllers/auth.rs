//! Module which contains all the HTTP REST endpoints related with user authentication and authorization

use std::sync::Arc;

use rocket::serde::json::Json;

use crate::{api_rest::dto::RegisterRequest, domain::user::service::user_service::UserService};

#[post("/sign-up", format = "json", data = "<new_user>")]
pub fn register(new_user: Json<RegisterRequest>) -> String {
    // TODO: provisional until set up DI via rocket ctx
    let user_service = crate::application::user::user_service::UserServiceImpl {
        user_repository: 
            Arc::new(Box::new(crate::infrastructure::persistence::relational::user::dao::user_repository_impl::UserRepositoryImpl))
    };
    user_service.register_user(&new_user.0); // TODO: bring anyhow to propagate errors
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