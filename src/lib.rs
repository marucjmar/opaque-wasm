#[cfg(feature = "client")]
pub mod client_login;
#[cfg(feature = "client")]
pub mod client_registration;
#[cfg(feature = "server")]
pub mod handle_login;
#[cfg(feature = "server")]
pub mod handle_registration;
#[cfg(feature = "server")]
pub mod server_setup;

mod hash_methods;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "client")]
pub use client_login::Login;
#[cfg(feature = "client")]
pub use client_registration::Registration;
#[cfg(feature = "server")]
pub use handle_login::HandleLogin;
#[cfg(feature = "server")]
pub use handle_registration::HandleRegistration;
#[cfg(feature = "server")]
pub use server_setup::ServerSetup;
