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

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_exchange() {
        // Server configuration
        let server_setup = ServerSetup::new();

        // Client configuration
        let username = "alice@example.com";
        let password = "correct horse battery staple";

        // Registration
        let [password_file, registration_export_key] = {
            let mut client_registration = Registration::new();
            let registration_request = client_registration.start(&password).unwrap();
            let server_registration = HandleRegistration::new(&server_setup);
            let registration_response = server_registration
                .start(username.into(), registration_request)
                .unwrap();
            let registration_record = client_registration
                .finish(&password, registration_response.clone())
                .unwrap();
            let password_file = server_registration
                .finish(registration_record.clone())
                .unwrap();
            assert_eq!(password_file, registration_record);
            let export_key = client_registration.get_export_key().unwrap();
            assert_ne!(export_key, registration_response);
            [password_file, export_key]
        };

        // Login
        let login_export_key = {
            let mut client_login = Login::new();
            let login_request = client_login.start(&password).unwrap();

            // Client -> Server - First request handler
            let mut server_login1 = HandleLogin::new(&server_setup);
            let login_response = server_login1
                .start(Some(password_file), username.into(), login_request)
                .unwrap();
            let serialized_state = server_login1.serialize().unwrap();
            // Client <- Server - end of first request handler

            let login_record = client_login.finish(&password, login_response).unwrap();
            let export_key = client_login.get_export_key().unwrap();
            let client_session_key = client_login.get_session_key().unwrap();
            assert_ne!(export_key, client_session_key);

            // Client -> Server - Second request handler
            let server_login2 = HandleLogin::deserialize(serialized_state, &server_setup).unwrap();
            let server_session_key = server_login2.finish(login_record).unwrap();
            assert_eq!(client_session_key, server_session_key);

            export_key
        };

        assert_eq!(
            registration_export_key, login_export_key,
            "Export keys differ"
        );
    }
}
