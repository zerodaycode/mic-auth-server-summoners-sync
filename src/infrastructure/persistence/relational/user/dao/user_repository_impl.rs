use canyon_sql::date_time::Utc;

use crate::domain::user::repository::user_repository::UserRepository;

pub struct UserRepositoryImpl;

// One option that would be amazing could be to store a user within the repo, as a DAO
// but unfortunately, with canyon we don't have such option
// other could be to require via fn parameter a fresh new instance of the target type

// The idea of passing it via parameter is that, we can generify the entrance and just
// use a trait bound, allowing database mocks to be up
impl UserRepository for UserRepositoryImpl {
    fn register_user(&self, new_user: &crate::api_rest::dto::RegisterRequest) -> Result<(), &'static dyn std::error::Error> {
        let bcrypted_pass = "some_mock_for_now_pass";
        let user_record = crate::infrastructure::persistence::relational::user::records::user::User {
            id: Default::default(),
            username: new_user.username.clone(),
            password: bcrypted_pass.to_owned(),
            email: new_user.email.clone(),
            created_at: Utc::now().naive_utc(),
            role_id: 1
        };
        println!("Saving user: {:?}", user_record);
        return Ok(()) // TODO: provisional, obviously
    }
}