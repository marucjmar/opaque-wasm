use digest::Digest;
use generic_array::typenum::Unsigned;
use generic_array::GenericArray;
use opaque_ke::errors::InternalPakeError;
use opaque_ke::hash::Hash;
use opaque_ke::slow_hash::SlowHash;

pub struct Scrypt;

impl<D: Hash> SlowHash<D> for Scrypt {
  fn hash(
    input: GenericArray<u8, <D as Digest>::OutputSize>,
  ) -> Result<Vec<u8>, InternalPakeError> {
    let params = scrypt::ScryptParams::new(15, 8, 1).unwrap();
    let mut output = vec![0u8; <D as Digest>::OutputSize::to_usize()];
    scrypt::scrypt(&input, &[], &params, &mut output)
      .map_err(|_| InternalPakeError::SlowHashError)?;
    Ok(output)
  }
}
