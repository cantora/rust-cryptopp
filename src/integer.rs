use libc::{c_void, c_long, size_t};

use cpp;

pub struct Integer {
  ctx: *mut c_void
}

impl Drop for Integer {
  fn drop(&mut self) {
    unsafe { cpp::del_Integer(self.ctx) };
  }
}

impl Clone for Integer {
  fn clone(&self) -> Integer {
    let ctx = unsafe { cpp::copy_Integer(self.ctx) };
    Integer { ctx: ctx }
  }
}

impl Integer {
  pub fn new() -> Integer {
    let ctx = unsafe {
      cpp::new_Integer()
    };

    Integer { ctx: ctx }
  }

  pub fn from_i32(val: i32) -> Integer {
    let ctx = unsafe { cpp::new_from_long_Integer(val as c_long) };
    Integer { ctx: ctx }
  }
}

pub fn new() -> Integer {
  Integer::new()
}


#[cfg(test)]
mod test {

  #[test]
  fn sanity() {
    let i = super::new();
    println!("i = {:?}", i.ctx);

    let i2 = i.clone();
    println!("i2 = {:?}", i2.ctx);

    let i3 = super::Integer::from_i32(34);
    println!("i3 = {:?}", i3.ctx);
  }

//  #[cfg(test)]
//  mod ctor {
//    fn long() {
//      let i = super::Integer::from_i32();
//    }
//  }
}
