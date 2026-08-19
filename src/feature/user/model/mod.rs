// Here common pattern in Rust known as a "facade."
// It allows you to organize your code into many small files for maintainability
//  while providing a clean, flat API for other parts of your application to use.
mod add_user_request;
mod unautheticated_user;
mod update_user_request;
mod user; // Declares the user.rs file as a module
mod user_claims;
mod user_row;

pub use self::add_user_request::AddUserRequest;
pub use self::unautheticated_user::UnauthenticatedUser;
pub use self::update_user_request::UpdateUserRequest;
pub use self::user::User; // Re-exports the User struct to the parent level
pub use self::user_claims::UserClaims;
pub use self::user_row::UserRow;
