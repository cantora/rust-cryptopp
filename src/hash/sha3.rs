
macro_rules! to_expr { ($e:expr) => ($e) }

macro_rules! define_sha3 {
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

define_sha3!("/SHA3_224.rs", h224, 28);
define_sha3!("/SHA3_256.rs", h256, 32);
define_sha3!("/SHA3_384.rs", h384, 48);
define_sha3!("/SHA3_512.rs", h512, 64);

#[cfg(test)]
mod test {
  use hash::DigestSize;
  use hash::Transformation;
  use hash::Function;
  use hash::Digest32;

  #[test]
  fn sanity() {
    let mut h256 = super::h256::new();
    let msg      = b"abc";
    let expected = Digest32::from_array([
                     0x4e, 0x03, 0x65, 0x7a, 0xea, 0x45, 0xa9, 0x4f,
                     0xc7, 0xd4, 0x7b, 0xa8, 0x26, 0xc8, 0xd6, 0x67,
                     0xc0, 0xd1, 0xe6, 0xe3, 0x3a, 0x64, 0xa0, 0x36,
                     0xec, 0x44, 0xf5, 0x8f, 0xa1, 0x2d, 0x6c, 0x45
                   ]);

    assert_eq!(h256.size(), DigestSize::Bits256);

    h256.update(msg);
    assert_eq!(h256.final_digest(), expected);

    assert_eq!(super::h256::digest(msg), expected);

    let msg2 = b"uchk uchk chk uchk ucka chka chuk";
    let expected2 = Digest32::from_array([
                      0xc5, 0x1a, 0x46, 0x40, 0x69, 0x4d, 0x14, 0x91,
                      0x6a, 0x82, 0xdd, 0xd6, 0x66, 0xd4, 0xea, 0x63,
                      0x15, 0x87, 0x45, 0xed, 0x99, 0xe6, 0xca, 0xd1,
                      0x33, 0x1f, 0x39, 0xc5, 0x7e, 0x3a, 0xbe, 0x37
                    ]);

    assert_eq!(super::h256::digest(msg2), expected2);
  }


  #[test]
  fn digest_empty_digest() {
    let empty_hash = Digest32::from_array([
                       0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c,
                       0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03, 0xc0,
                       0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b,
                       0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85, 0xa4, 0x70
                     ]);
    assert_eq!(super::h256::empty_digest(), empty_hash);
  }

}
