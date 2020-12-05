mod hash_methods;
mod utils;

use hash_methods::Scrypt;
use opaque_ke::ciphersuite::CipherSuite;
use opaque_ke::{
    ClientLogin, ClientLoginFinishParameters, ClientLoginStartParameters, ClientRegistration,
    ClientRegistrationStartParameters, LoginSecondMessage, RegisterSecondMessage,
};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;

struct Default;
impl CipherSuite for Default {
    type Group = curve25519_dalek::ristretto::RistrettoPoint;
    type KeyFormat = opaque_ke::keypair::X25519KeyPair;
    type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDH;
    type Hash = sha2::Sha256;
    type SlowHash = Scrypt;
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Registration {
    state: Option<ClientRegistration<Default>>,
    rng: OsRng,
}

#[wasm_bindgen]
impl Registration {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Registration {
        Registration {
            rng: OsRng,
            state: None,
        }
    }

    pub fn start(&mut self, password: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let (message, client_state) = ClientRegistration::<Default>::start(
            &password,
            ClientRegistrationStartParameters::default(),
            &mut self.rng,
        )
        .unwrap();
        self.state = Some(client_state);

        return Ok(message.serialize());
    }

    pub fn finish(self, message: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let message = RegisterSecondMessage::deserialize(&message[..]);
        if message.is_err() {
            return Err("Message deserialize failed".into());
        }
        let mut rng = self.rng;

        let (message, _s) = self
            .state
            .unwrap()
            .finish(message.unwrap(), &mut rng)
            .unwrap();
        return Ok(message.serialize());
    }
}

#[wasm_bindgen]
pub struct Login {
    state: Option<ClientLogin<Default>>,
    rng: OsRng,
}

#[wasm_bindgen]
impl Login {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Login {
        Login {
            rng: OsRng,
            state: None,
        }
    }

    pub fn start(&mut self, password: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let client_login_start_result = ClientLogin::<Default>::start(
            &password,
            &mut self.rng,
            ClientLoginStartParameters::default(),
        );

        if client_login_start_result.is_err() {
            return Err("Failed start".into());
        }

        let result = client_login_start_result.unwrap();

        self.state = Some(result.client_login_state);

        return Ok(result.credential_request.serialize());
    }

    pub fn finish(self, message: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let message = LoginSecondMessage::deserialize(&message[..]);

        if message.is_err() {
            return Err("Message deserialize failed".into());
        }

        let result = match self
            .state
            .unwrap()
            .finish(message.unwrap(), ClientLoginFinishParameters::default())
        {
            Ok(result) => result,
            Err(_e) => return Err("Mismatch messagess".into()),
        };

        return Ok(result.key_exchange.serialize());
    }
}
