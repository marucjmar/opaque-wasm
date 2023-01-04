use crate::hash_methods::Default;
use wasm_bindgen::prelude::*;
use rand::rngs::OsRng;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ServerSetup {
    internal: opaque_ke::ServerSetup<Default>
}

#[wasm_bindgen]
impl ServerSetup {

    #[wasm_bindgen(constructor)]
    pub fn new() -> ServerSetup {
        let mut rng = OsRng;
        let internal = opaque_ke::ServerSetup::new(&mut rng);

        ServerSetup { internal }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.internal.serialize().to_vec()
    }

    pub fn deserialize(input: Vec<u8>) -> Result<ServerSetup, JsValue> {
        let internal  = match opaque_ke::ServerSetup::deserialize(&input) {
            Ok(val) => val,
            Err(_) => return Err("Failed to load serialized ServerSetup".into())
        };
        Ok(
            ServerSetup {
                internal
            }
        )
    }

    pub(crate) fn internal<'a>(&'a self) -> &'a opaque_ke::ServerSetup<Default> {
        &self.internal
    }
}