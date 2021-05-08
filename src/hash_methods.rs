use opaque_ke::ciphersuite::CipherSuite;

pub struct Default;
impl CipherSuite for Default {
  type Group = curve25519_dalek::ristretto::RistrettoPoint;
  type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDH;
  type Hash = sha2::Sha512;
  type SlowHash = scrypt::ScryptParams;
}
