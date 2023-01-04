use opaque_ke::ciphersuite::CipherSuite;

pub struct Default;
impl CipherSuite for Default {
    type OprfCs = p256::NistP256;
    type KeGroup = p256::NistP256;
    type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDh;
    type Ksf = argon2::Argon2<'static>;
}