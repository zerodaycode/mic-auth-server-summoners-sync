use crate::api_rest::dto::RegisterRequest;

/// The public `API` for handling user authentication, authorization and registration
pub trait UserService {
    fn register_user(new_user: RegisterRequest) -> Result<(), &'static dyn std::error::Error>;
}

