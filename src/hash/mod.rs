use libc::{size_t};
use std::default::Default;

use cpp;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DigestSize {
  Bits160,
  Bits224,
  Bits256,
  Bits384,
  Bits512
}

impl DigestSize {
  pub fn in_bits(&self) -> u32 {
    self.in_bytes()*8
  }

  /// returns digest size in bytes.
  pub fn in_bytes(&self) -> u32 {
    use self::DigestSize::*;

    match self {
      &Bits160 => 20,
      &Bits224 => 28,
      &Bits256 => 32,
      &Bits384 => 48,
      &Bits512 => 64
    }
  }

  fn from_size_in_bytes(bytes: u32) -> DigestSize {
    DigestSize::from_size_in_bits(bytes*8)
  }

  fn from_size_in_bits(bits: u32) -> DigestSize {
    match bits {
      160 => DigestSize::Bits160,
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
pub trait Transformation : cpp::CPPContext {
  /// updates the hash function state with input data.
  fn update(&mut self, data: &[u8]) {
    unsafe {
      cpp::mth_HashTransformation_Update(self.mut_ctx(),
                                         data.as_ptr(),
                                         data.len() as size_t)
    };
  }

  /// returns the digest and resets the hash function state.
  fn finalize(&mut self, output: &mut [u8]) {
    assert!(output.len() >= self.size().in_bytes() as usize);

    unsafe {
      cpp::mth_HashTransformation_Final(self.mut_ctx(), output.as_mut_ptr())
    };
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

//TODO: remove Eq and replace it with a constant time Eq trait
pub trait Digest : Default + AsMut<[u8]> + Eq + Debug {
  fn size() -> DigestSize;
}

macro_rules! define_digest_type {
  ($modname:ident, $tname:ident , $sz:expr) => (
    mod $modname {
      use std::{slice, fmt, hash, cmp};

      array_wrap!($tname, u8, $sz, Default
                                   From_array
                                   Into_array
                                   AsRef_array
                                   AsMut_array
                                   AsRef_slice
                                   AsMut_slice
                                   Hash
                                   Debug
                                   IntoIterator
                                   PartialEq
                                   Eq
                                   PartialOrd
                                   Ord);

      impl super::Digest for $tname {
        fn size() -> super::DigestSize {
          super::DigestSize::from_size_in_bytes($sz)
        }
      }

      #[cfg(test)]
      mod test {
        #[test]
        fn sanity() {
          use super::super::Digest;
          assert_eq!(super::$tname::default().len(),
                     super::$tname::size().in_bytes() as usize);
        }
      }
    }

    pub use self::$modname::$tname;
  )
}

define_digest_type!(digest20, Digest20, 20);
define_digest_type!(digest28, Digest28, 28);
define_digest_type!(digest32, Digest32, 32);
define_digest_type!(digest48, Digest48, 48);
define_digest_type!(digest64, Digest64, 64);

//this relationship is checked by the `size_sanity` test in generated hash functions
macro_rules! size_to_output_type {
  (20) => (type Output = hash::Digest20;);
  (28) => (type Output = hash::Digest28;);
  (32) => (type Output = hash::Digest32;);
  (48) => (type Output = hash::Digest48;);
  (64) => (type Output = hash::Digest64;);
}

use std::fmt::Debug;
pub trait Function : Transformation + Default {
  type Output : Digest;

  fn final_digest(&mut self) -> Self::Output {
    let mut output = Self::Output::default();
    self.finalize(output.as_mut());
    output
  }

  fn digest(data: &[u8]) -> Self::Output {
    let hash_fn = &mut Self::default();
    hash_fn.update(data);
    hash_fn.final_digest()
  }

  fn empty_digest() -> Self::Output {
    Self::digest(b"")
  }
}

#[cfg(test)]
mod test {

  #[test]
  fn digest_size_sanity() {
    use super::DigestSize as DS;

    let ds1 = DS::from_size_in_bits(224);
    assert_eq!(ds1, DS::Bits224);
    assert_eq!(ds1.in_bits(), 224);
    assert_eq!(ds1.in_bytes(), 28);

    let ds2 = DS::from_size_in_bytes(32);
    assert_eq!(ds2, DS::Bits256);
    assert_eq!(ds2.in_bits(), 256);
    assert_eq!(ds2.in_bytes(), 32);
  }

  pub mod digest {
    use hash;

    pub fn reset<T: hash::Function>() {
      let mut d = T::default();
      d.reset();

      assert_eq!(d.final_digest(), T::empty_digest());

      d.update(b"70 72 6e b7 9a 31 54 5c  a6 b8 b7 94 5e c2 ea e5");
      d.reset();
      assert_eq!(d.final_digest(), T::empty_digest());
    }

    pub fn finalize<T: hash::Function>() {
      let mut d = T::default();
      d.reset();
      assert_eq!(d.final_digest(), T::empty_digest());

      d.update(b"asdofijqwoeirj");
      d.final_digest();

      assert_eq!(d.final_digest(), T::empty_digest());
    }

    pub fn update<T: hash::Function>() {
      let mut d = T::default();
      d.reset();
      assert_eq!(d.final_digest(), T::empty_digest());

      d.update(b"");
      assert_eq!(d.final_digest(), T::empty_digest());
    }
  }


}

macro_rules! to_expr { ($e:expr) => ($e) }

macro_rules! define_sized_hash_module {
  ($file:expr, $modname:ident, $hsize:tt) => (
    pub mod $modname {
      use cpp;
      use libc::{c_void};
      use hash;
      use hash::{Transformation, Function};

      include!(concat!(env!("OUT_DIR"), $file));

      impl Transformation for Hash {}

      impl Function for Hash {
        //size_to_arr_digest_result!(u8, $hsize);
        size_to_output_type!($hsize);
      }

      pub fn new() -> Hash {
        Hash::new()
      }

      pub fn digest(msg: &[u8]) -> <Hash as Function>::Output {
        Hash::digest(msg)
      }

      pub fn empty_digest() -> <Hash as Function>::Output {
        Hash::empty_digest()
      }

      #[cfg(test)]
      mod test {
        use hash;

        #[test]
        fn size_sanity() {
          use super::Hash;
          use hash::Digest;

          let dsize = hash::DigestSize::from_size_in_bytes(to_expr!($hsize));
          assert_eq!(dsize, <Hash as hash::Function>::Output::size());
        }

        #[test]
        fn digest_tests() {
          hash::test::digest::reset::<super::Hash>();
          hash::test::digest::finalize::<super::Hash>();
          hash::test::digest::update::<super::Hash>();
        }
      }

    }
  )
}

// sub-modules must be defined down here to ensure macros are visible
define_sized_hash_module!("/SHA1.rs", sha1, 20);
pub mod sha3;
