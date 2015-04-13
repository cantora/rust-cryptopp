use libc::{c_void, size_t};

use cpp;

pub struct Sha3 {
  ctx: *mut c_void
}

impl Drop for Sha3 {
  fn drop(&mut self) {
    unsafe { cpp::SHA3_256_delete(self.ctx) };
  }
}

impl Sha3 {
  pub fn new() -> Sha3 {
    let ctx = unsafe {
      cpp::SHA3_256_new()
    };

    Sha3 { ctx: ctx }
  }

  pub fn update(&mut self, data: &[u8]) {
    unsafe {
      cpp::HashTransformation_Update(self.ctx,
                                     data.as_ptr(),
                                     data.len() as size_t)
    };
  }

  pub fn digest(&mut self) -> [u8; 32] {
    let mut output = [0; 32];
    unsafe {
      cpp::HashTransformation_Final(self.ctx, output.as_mut_ptr())
    };
    output
  }
}


pub fn new() -> Sha3 {
  Sha3::new()
}

#[cfg(test)]
mod test {
  extern crate rustc_serialize;
  use self::rustc_serialize::hex::FromHex;

  #[test]
  fn sanity() {
    let mut sha3 = super::new();
    let msg      = b"abc";
    let expected = FromHex::from_hex(
      "4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45"
    ).unwrap();

    sha3.update(msg);
    let dgst = sha3.digest();

    assert_eq!(&dgst[..], &expected[..]);
  }
}
