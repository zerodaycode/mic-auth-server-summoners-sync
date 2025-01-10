use std::sync::Arc;

use crate::domain::user::{repository::user_repository::UserRepository, service::user_service::UserService};

pub struct UserServiceImpl {
    // TODO: remember to remove the pub and use ctr injection
    pub user_repository: Arc<Box<dyn UserRepository>>
}

impl UserService for UserServiceImpl {
    fn register_user(&self, new_user: &crate::api_rest::dto::RegisterRequest) -> Result<(), &'static dyn std::error::Error> {
        self.user_repository.register_user(new_user)
    }
}