use crate::hash_methods::Default;
use crate::server_setup::ServerSetup;
use opaque_ke::{
    CredentialFinalization, CredentialRequest, ServerLogin,
    ServerLoginStartParameters, ServerRegistration
};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HandleLogin {
    setup: ServerSetup,
    state: Option<ServerLogin<Default>>,
    rng: OsRng,
}

#[wasm_bindgen]
impl HandleLogin {
    #[wasm_bindgen(constructor)]
    pub fn new(setup: &ServerSetup) -> HandleLogin {
        HandleLogin {
            setup: setup.clone(),
            state: None,
            rng: OsRng,
        }
    }

    pub fn start(
        &mut self,
        password_file: Vec<u8>,
        identifier: Vec<u8>,
        credential_request: Vec<u8>,
    ) -> Result<Vec<u8>, JsValue> {
        let request = CredentialRequest::deserialize(&credential_request[..]).unwrap();
        let password = ServerRegistration::<Default>::deserialize(&password_file[..]).unwrap();

        let server_login_start_result = match ServerLogin::start(
            &mut self.rng,
            self.setup.internal(),
            Some(password),
            request,
            &identifier,
            ServerLoginStartParameters::default(),
        ) {
            Ok(message) => message,
            Err(_e) => return Err("Message deserialize failed".into()),
        };

        self.state = Some(server_login_start_result.state);

        return Ok(server_login_start_result.message.serialize());
    }

    pub fn finish(self, credential_finish: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let finish = CredentialFinalization::deserialize(&credential_finish[..]).unwrap();

        let result = self.state.unwrap().finish(finish).unwrap();
        return Ok(result.session_key);
    }
}
