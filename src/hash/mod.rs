use libc::{size_t};
use std::default::Default;

use cpp;

pub mod sha3;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DigestSize {
  Bits224,
  Bits256,
  Bits384,
  Bits512
}

impl DigestSize {
  pub fn in_bits(&self) -> u32 {
    use self::DigestSize::*;

    match self {
      &Bits224 => 224,
      &Bits256 => 256,
      &Bits384 => 384,
      &Bits512 => 512
    }
  }

  /// returns digest size in bytes.
  /// None is returned if the size in bits is not divisible by eight.
  pub fn in_bytes(&self) -> Option<u32> {
    use self::DigestSize::*;

    Some(match self {
      &Bits224 => 224/8,
      &Bits256 => 256/8,
      &Bits384 => 384/8,
      &Bits512 => 512/8
    })
  }

  fn from_size_in_bytes(bytes: u32) -> DigestSize {
    DigestSize::from_size_in_bits(bytes << 3)
  }

  fn from_size_in_bits(bits: u32) -> DigestSize {
    match bits {
      224 => DigestSize::Bits224,
      256 => DigestSize::Bits256,
      384 => DigestSize::Bits384,
      512 => DigestSize::Bits512,
      // if we support a hash that has a size other than
      // those listed here then its a bug
      _  => unreachable!()
    }
  }
}

/// things that satisfy the hash transformation interface.
pub trait Function : cpp::CPPContext {
  /// updates the hash function state with input data.
  fn update(&mut self, data: &[u8]) {
    unsafe {
      cpp::mth_HashTransformation_Update(self.mut_ctx(),
                                         data.as_ptr(),
                                         data.len() as size_t)
    };
  }

  /// returns the digest and resets the hash function state.
  fn finalize(&mut self) -> [u8; 32] {
    let mut output = [0; 32];
    unsafe {
      cpp::mth_HashTransformation_Final(self.mut_ctx(), output.as_mut_ptr())
    };
    output
  }

  // reset hash function state
  fn reset(&mut self) {
    unsafe {
      cpp::mth_HashTransformation_Restart(self.mut_ctx())
    };
  }

  /// the digest size.
  fn size(&self) -> DigestSize {
    DigestSize::from_size_in_bytes(unsafe {
      cpp::mth_HashTransformation_DigestSize(self.ctx())
    })
  }
}

/// a digest is a function that only takes input data and no other
/// parameters.
pub trait Digest : Function + Default {
  fn digest(data: &[u8]) -> [u8; 32] {
    let hash_fn = &mut Self::default();
    hash_fn.update(data);
    hash_fn.finalize()
  }

  fn empty_digest() -> [u8; 32] {
    Self::digest(b"")
  }
}

#[cfg(test)]
mod test {
  use super::Digest;
  use super::Function;

  #[test]
  fn digest_size_sanity() {
    use super::DigestSize as DS;

    let ds1 = DS::from_size_in_bits(224);
    assert_eq!(ds1, DS::Bits224);
    assert_eq!(ds1.in_bits(), 224);
    assert_eq!(ds1.in_bytes().unwrap(), 28);

    let ds2 = DS::from_size_in_bytes(32);
    assert_eq!(ds2, DS::Bits256);
    assert_eq!(ds2.in_bits(), 256);
    assert_eq!(ds2.in_bytes().unwrap(), 32);
  }

  pub fn reset<T: Digest>(mut d: T) {
    d.reset();
    assert_eq!(d.finalize(), T::empty_digest());

    d.update(b"    assert_eq!(d.finalize(), T::empty_digest());");
    d.reset();
    assert_eq!(d.finalize(), T::empty_digest());
  }

  pub fn finalize<T: Digest>(mut d: T) {
    d.reset();
    assert_eq!(d.finalize(), T::empty_digest());

    d.update(b"asdofijqwoeirj");
    d.finalize();

    assert_eq!(d.finalize(), T::empty_digest());
  }

  pub fn update<T: Digest>(mut d: T) {
    d.reset();
    assert_eq!(d.finalize(), T::empty_digest());

    d.update(b"");
    assert_eq!(d.finalize(), T::empty_digest());
  }

}
