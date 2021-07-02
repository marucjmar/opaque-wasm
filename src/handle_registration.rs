use crate::hash_methods::Default;
use opaque_ke::{RegistrationRequest, RegistrationUpload, ServerRegistration, ServerSetup};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HandleRegistration {
}

#[wasm_bindgen]
impl HandleRegistration {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HandleRegistration {
        HandleRegistration {
        }
    }

    pub fn start(
        &mut self,
        identifier: Vec<u8>,
        registration_request: Vec<u8>,
        server_setup: Vec<u8>,
    ) -> Result<Vec<u8>, JsValue> {
        let server_setup = ServerSetup::deserialize(&server_setup).unwrap();
        let request = match RegistrationRequest::deserialize(&registration_request[..]) {
            Ok(message) => message,
            Err(_e) => return Err("Message deserialize failed".into()),
        };

        let server_registration_start_result =
            match ServerRegistration::<Default>::start(&server_setup, request, &identifier)
            {
                Ok(message) => message,
                Err(_e) => return Err("Message deserialize failed".into()),
            };

        return Ok(server_registration_start_result.message.serialize());
    }

    pub fn finish(self, registration_finish: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let message = RegistrationUpload::deserialize(&registration_finish[..]).unwrap();
        let password_file = ServerRegistration::<Default>::finish(message);

        return Ok(password_file.serialize());
    }
}
