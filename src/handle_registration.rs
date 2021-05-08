use crate::hash_methods::Default;
use opaque_ke::{keypair::KeyPair, RegistrationRequest, RegistrationUpload, ServerRegistration};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HandleRegistration {
  rng: OsRng,
  state: Option<ServerRegistration<Default>>,
}

#[wasm_bindgen]
impl HandleRegistration {
  #[wasm_bindgen(constructor)]
  pub fn new() -> HandleRegistration {
    HandleRegistration {
      rng: OsRng,
      state: None,
    }
  }

  pub fn start(
    &mut self,
    registration_request: Vec<u8>,
    server_privatekey: Vec<u8>,
  ) -> Result<Vec<u8>, JsValue> {
    let server_kp = KeyPair::<curve25519_dalek::ristretto::RistrettoPoint>::from_private_key_slice(
      &server_privatekey,
    )
    .unwrap();
    let request = match RegistrationRequest::deserialize(&registration_request[..]) {
      Ok(message) => message,
      Err(_e) => return Err("Message deserialize failed".into()),
    };

    let server_registration_start_result =
      match ServerRegistration::<Default>::start(&mut self.rng, request, &server_kp.public()) {
        Ok(message) => message,
        Err(_e) => return Err("Message deserialize failed".into()),
      };

    self.state = Some(server_registration_start_result.state);
    return Ok(server_registration_start_result.message.serialize());
  }

  pub fn finish(self, registration_finish: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let message = RegistrationUpload::deserialize(&registration_finish[..]).unwrap();
    let password_file = self.state.unwrap().finish(message).unwrap();

    return Ok(password_file.serialize());
  }
}
