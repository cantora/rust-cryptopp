use libc::{c_void, size_t};

use cpp;

include!(concat!(env!("OUT_DIR"), "/SHA3_256.rs"));

pub trait CPPContext {
  fn mut_ctx(&self) -> *mut c_void;
  fn ctx(&self) -> *const c_void {
    self.mut_ctx()
  }
}

impl CPPContext for Sha3 {
  fn mut_ctx(&self) -> *mut c_void {
    self.ctx
  }
}

pub trait HashTransformation : CPPContext {
  fn update(&mut self, data: &[u8]) {
    unsafe {
      cpp::mth_HashTransformation_Update(self.mut_ctx(),
                                         data.as_ptr(),
                                         data.len() as size_t)
    };
  }

  fn digest(&mut self) -> [u8; 32] {
    let mut output = [0; 32];
    unsafe {
      cpp::mth_HashTransformation_Final(self.mut_ctx(), output.as_mut_ptr())
    };
    output
  }
}

impl HashTransformation for Sha3 {}

pub fn new() -> Sha3 {
  Sha3::new()
}

pub fn digest(msg: &[u8]) -> [u8; 32] {
  let mut sha3 = Sha3::new();
  sha3.update(msg);
  sha3.digest()
}

/// produce a keccak hmac; that is, an hmac done insecurely but its
/// ok because keccak is crazy like that.
/// http://en.wikipedia.org/wiki/Hash-based_message_authentication_code
/// the hmac is: `keccak(secret || msg)`
/// maybe dangerous?
///
/// # Panics
/// - when secret.len() < 16
/// - when msg is empty
pub fn keccak_mac(secret: &[u8], msg: &[u8]) -> [u8; 32] {
  assert!(secret.len() >= 16, "secret is dangerously small");
  assert!(msg.len() > 0, "msg is empty");

  let mut keccak = Sha3::new();

  keccak.update(secret);
  keccak.update(msg);
  keccak.digest()
}

#[cfg(test)]
mod test {
  extern crate rustc_serialize;
  use self::rustc_serialize::hex::FromHex;
  use super::HashTransformation;

  #[test]
  fn sanity() {
    let mut sha3 = super::new();
    let msg      = b"abc";
    let expected = FromHex::from_hex(
      "4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45"
    ).unwrap();
    let expected_bs = &expected[..];

    sha3.update(msg);
    let dgst = sha3.digest();

    assert_eq!(&dgst[..], expected_bs);

    assert_eq!(super::digest(msg), expected_bs);

    let msg2 = b"uchk uchk chk uchk ucka chka chuk";
    let expected2 = FromHex::from_hex(
      "c51a4640694d14916a82ddd666d4ea63158745ed99e6cad1331f39c57e3abe37"
    ).unwrap();
    let expected2_bs = &expected2[..];

    assert_eq!(super::digest(msg2), expected2_bs);    

    assert_eq!(super::keccak_mac(&msg2[..16], &msg2[16..]),
               expected2_bs);
  }

  mod keccak_mac {
    #[test]
    #[should_panic]
    fn panic_secret() {
      super::super::keccak_mac(b"too short", b"msg");
    }

    #[test]
    #[should_panic]
    fn panic_msg() {
      super::super::keccak_mac(b"0123456789012345", b"");
    }
  }
}
