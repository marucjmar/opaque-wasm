use crate::hash_methods::Default;
use opaque_ke::{
    keypair::KeyPair, CredentialFinalization, CredentialRequest, ServerLogin,
    ServerLoginStartParameters, ServerRegistration,
};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HandleLogin {
    state: Option<ServerLogin<Default>>,
    rng: OsRng,
}

#[wasm_bindgen]
impl HandleLogin {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HandleLogin {
        HandleLogin {
            state: None,
            rng: OsRng,
        }
    }

    pub fn start(
        &mut self,
        password_file: Vec<u8>,
        credential_request: Vec<u8>,
        server_privatekey: Vec<u8>,
    ) -> Result<Vec<u8>, JsValue> {
        let server_kp =
            KeyPair::<curve25519_dalek::ristretto::RistrettoPoint>::from_private_key_slice(
                &server_privatekey,
            )
            .unwrap();

        let request = CredentialRequest::deserialize(&credential_request[..]).unwrap();
        let password = ServerRegistration::<Default>::deserialize(&password_file[..]).unwrap();

        let server_login_start_result = match ServerLogin::start(
            &mut self.rng,
            password,
            &server_kp.private(),
            request,
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
