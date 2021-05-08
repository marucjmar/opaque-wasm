pub mod client_login;
pub mod client_registration;
pub mod handle_login;
pub mod handle_registration;
mod hash_methods;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use client_login::Login;
pub use client_registration::Registration;
pub use handle_login::HandleLogin;
pub use handle_registration::HandleRegistration;
