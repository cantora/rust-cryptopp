use libc::{c_void, size_t};

use cpp;

pub mod sha3;

pub trait Function : cpp::CPPContext {
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
