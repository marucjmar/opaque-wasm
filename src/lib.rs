mod hash_methods;
mod utils;

use hash_methods::Scrypt;
use opaque_ke::{
    ClientLogin, ClientLoginFinishParameters, ClientLoginStartParameters, ClientRegistration,
    CredentialResponse, RegistrationResponse, ClientRegistrationFinishParameters, ServerRegistration,
    ServerLogin, ServerLoginStartParameters, RegistrationRequest, CredentialRequest, RegistrationUpload,
    CredentialFinalization, keypair::KeyPair, ciphersuite::CipherSuite
};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;

struct Default;
impl CipherSuite for Default {
    type Group = curve25519_dalek::ristretto::RistrettoPoint;
    type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDH;
    type Hash = sha2::Sha512;
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
        let client_registration_start_result = match ClientRegistration::<Default>::start(
            &mut self.rng,
            &password,
        ) {
            Ok(reply) => reply,
            Err(_e) => return Err("Start failed".into()),
        };
        self.state = Some(client_registration_start_result.state);

        return Ok(client_registration_start_result.message.serialize());
    }

    pub fn finish(self, message: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let message = match RegistrationResponse::deserialize(&message[..]) {
            Ok(message) => message,
            Err(_e) => return Err("Message deserialize failed".into()),
        };
        let mut rng = self.rng;

        let client_finish_registration_result = match self.state.unwrap().finish(&mut rng, message, ClientRegistrationFinishParameters::default()) {
            Ok(reply) => reply,
            Err(_e) => return Err("Mismatch messagess".into()),
        };
        return Ok(client_finish_registration_result.message.serialize());
    }
}

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

    pub fn start (&mut self, registration_request: Vec<u8>, server_privatekey: Vec<u8>) -> Result<Vec<u8>, JsValue> {

        let server_kp = KeyPair::<curve25519_dalek::ristretto::RistrettoPoint>::from_private_key_slice(&server_privatekey).unwrap();
        let request = match RegistrationRequest::deserialize(&registration_request[..])
        {
            Ok(message) => message,
            Err(_e) => return Err("Message deserialize failed".into()),
        };

        let server_registration_start_result = match ServerRegistration::<Default>::start(
            &mut self.rng,
            request,
            &server_kp.public(),
        ){
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

    pub fn start(&mut self, password: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let client_login_start_result = match ClientLogin::<Default>::start(
            &mut self.rng,
            &password,
            ClientLoginStartParameters::default(),
        ) {
            Ok(client_login_start_result ) => client_login_start_result ,
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

        let result = state.unwrap().finish(message.unwrap(), ClientLoginFinishParameters::default()).unwrap();

        self.session_key = Some(result.session_key);

        return Ok(result.message.serialize());
    }

    #[wasm_bindgen(js_name = getSessionKey)]
    pub fn get_session_key(self) -> Result<Vec<u8>, JsValue> {
        return Ok(self.session_key.unwrap());
    }
}

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

    pub fn start(&mut self, password_file: Vec<u8>, credential_request: Vec<u8>, server_privatekey: Vec<u8> ) -> Result<Vec<u8>, JsValue> {

        let server_kp = KeyPair::<curve25519_dalek::ristretto::RistrettoPoint>::from_private_key_slice(&server_privatekey).unwrap();

        let request = CredentialRequest::deserialize(&credential_request[..]).unwrap();
        let password = ServerRegistration::<Default>::deserialize(&password_file[..]).unwrap();

        let server_login_start_result = match ServerLogin::start(
            &mut self.rng,
            password,
            &server_kp.private(),
            request,
            ServerLoginStartParameters::default(),
        ){
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
