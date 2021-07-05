use crate::hash_methods::Default;
use crate::server_setup::ServerSetup;
use opaque_ke::{RegistrationRequest, RegistrationUpload, ServerRegistration};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HandleRegistration {
    setup: ServerSetup
}

#[wasm_bindgen]
impl HandleRegistration {
    #[wasm_bindgen(constructor)]
    pub fn new(setup: &ServerSetup) -> HandleRegistration {
        HandleRegistration {
            setup: setup.clone()
        }
    }

    pub fn start(
        &self,
        identifier: Vec<u8>,
        registration_request: Vec<u8>,
    ) -> Result<Vec<u8>, JsValue> {
        let request = match RegistrationRequest::deserialize(&registration_request[..]) {
            Ok(message) => message,
            Err(_e) => return Err("Message deserialize failed".into()),
        };

        let server_registration_start_result =
            match ServerRegistration::<Default>::start(self.setup.internal(), request, &identifier)
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
