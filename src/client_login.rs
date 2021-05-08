use crate::hash_methods::Default;
use opaque_ke::{
  ClientLogin, ClientLoginFinishParameters, ClientLoginStartParameters, CredentialResponse,
};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Login {
  state: Option<ClientLogin<Default>>,
  rng: OsRng,
  session_key: Option<Vec<u8>>,
}

#[wasm_bindgen]
impl Login {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Login {
    Login {
      rng: OsRng,
      state: None,
      session_key: None,
    }
  }

  pub fn start(&mut self, password: &str) -> Result<Vec<u8>, JsValue> {
    let client_login_start_result = match ClientLogin::<Default>::start(
      &mut self.rng,
      &password.as_bytes(),
      ClientLoginStartParameters::default(),
    ) {
      Ok(client_login_start_result) => client_login_start_result,
      Err(_e) => return Err("Failed start".into()),
    };

    self.state = Some(client_login_start_result.state);

    return Ok(client_login_start_result.message.serialize());
  }

  pub fn finish(&mut self, message: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let message = CredentialResponse::deserialize(&message[..]);

    if message.is_err() {
      return Err("Message deserialize failed".into());
    }

    let state = self.state.take();

    let result = state
      .unwrap()
      .finish(message.unwrap(), ClientLoginFinishParameters::default())
      .unwrap();

    self.session_key = Some(result.session_key);

    return Ok(result.message.serialize());
  }

  #[wasm_bindgen(js_name = getSessionKey)]
  pub fn get_session_key(self) -> Result<Vec<u8>, JsValue> {
    return Ok(self.session_key.unwrap());
  }
}
