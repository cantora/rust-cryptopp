extern crate libc;

#[cfg(test)]
#[macro_use]
extern crate nom;

#[macro_use]
pub mod arr;
pub mod hash;
pub mod integer;

mod cpp;

#[cfg(test)]
mod test;
