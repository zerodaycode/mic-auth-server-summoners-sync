use crate::api_rest::dto::RegisterRequest;

/// The public `API` for handling user authentication, authorization and registration
pub trait UserService {
    /// Persist a new [crate::domain::user::models::User] in the form of a [crate::infrastructure::records::User].
	///
	/// # Errors
	///
	/// - MUST return [RegisterUserError::Duplicate] if an [User] with the same [Username] and or [UserEmail]
	///   already exists.
    fn register_user(new_user: &RegisterRequest) -> Result<(), &'static dyn std::error::Error>;
}
