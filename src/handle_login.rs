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
        password_file: Option<Vec<u8>>,
        identifier: Vec<u8>,
        credential_request: Vec<u8>,
    ) -> Result<Vec<u8>, JsValue> {

        let request = CredentialRequest::deserialize(&credential_request[..]).unwrap();

        let password = match password_file {
            Some(val) => Some(ServerRegistration::<Default>::deserialize(&val).unwrap()),
            None => None
        };

        let server_login_start_result = match ServerLogin::start(
            &mut self.rng,
            self.setup.internal(),
            password,
            request,
            &identifier,
            ServerLoginStartParameters::default(),
        ) {
            Ok(message) => message,
            Err(_e) => return Err("Message deserialize failed".into()),
        };

        self.state = Some(server_login_start_result.state);

        return Ok(server_login_start_result.message.serialize().to_vec());
    }

    pub fn finish(self, credential_finish: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let finish = CredentialFinalization::deserialize(&credential_finish[..]).unwrap();

        let result = self.state.unwrap().finish(finish).unwrap();
        return Ok(result.session_key.to_vec());
    }

    pub fn serialize(&self) -> Result<Vec<u8>, JsValue> {
        match &self.state {
            Some(state) => Ok(state.serialize().to_vec()),
            None => Err("Failed to serialize ServerLogin (no state available)".into()),
        }
    }

    pub fn deserialize(
        serialized_state: Vec<u8>,
        setup: &ServerSetup,
    ) -> Result<HandleLogin, JsValue> {
        let state = match opaque_ke::ServerLogin::<Default>::deserialize(&serialized_state) {
            Ok(val) => val,
            Err(_) => return Err("Failed to load serialized ServerLogin".into()),
        };
        Ok(HandleLogin {
            setup: setup.clone(),
            state: Some(state),
            rng: OsRng,
        })
    }
}
