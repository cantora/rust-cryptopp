extern crate pkg_config;
extern crate gcc;

#[macro_use]
extern crate rust_cryptopp_gen as gen;

use std::io;
use std::io::Write;
use std::fs::File;
use std::convert::From;

fn find_cryptopp() -> pkg_config::Library {
  match pkg_config::find_library("libcrypto++") {
    Ok(info) => info,
    Err(err) => panic!("failed to find cryptopp: {}", err)
  }
}

#[derive(Debug)]
pub enum Error {
  IO(io::Error),
  Gen(gen::Error),
  Unexpected(String)
}

type Result<T> = std::result::Result<T, Error>;

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Error {
    Error::IO(e)
  }
}

impl From<gen::Error> for Error {
  fn from(e: gen::Error) -> Error {
    Error::Gen(e)
  }
}

fn gen_classes(mut ctx: gen::Context<File, File>,
               out_path: &std::path::Path) -> gen::Result<()> {
  use gen::proto::*;

  try!(ctx.generate(&class!(b"HashTransformation" => {
    mutable methods {
      void(), b"Update",     const_ptr(UChar), size_t();
      void(), b"Final",      mut_ptr(UChar);
      void(), b"Restart";
    }
    constant methods {
      uint(), b"DigestSize";
    }
  })));

  let sha3 = class!(b"SHA3_256" => {
    constructors {
      b"";
    }
  });
  try!(ctx.generate(&sha3));
  try!(sha3.generate_struct(out_path, b"Sha3"));

  ctx.generate(&class!(b"Integer" => {
    constructors {
      b"";
      b"copy",      const_ref(Custom(b"Integer"));
      b"from_long", long();
    }
  }))
}

fn gen_cpp_code(cpp_path: &std::path::Path,
                rust_binding_path: &std::path::Path,
                out_path: &std::path::Path) -> Result<()> {

  let mut cpp_stream = try!(File::create(cpp_path));

  try!(cpp_stream.write_all(b"#include <cryptopp/cryptlib.h>\n"));
  try!(cpp_stream.write_all(b"#include <cryptopp/sha3.h>\n"));
  try!(cpp_stream.write_all(b"#include <cryptopp/integer.h>\n"));
  try!(cpp_stream.write_all(b"using namespace CryptoPP;\n\n"));

  let mut rs_binding_stream = try!(File::create(rust_binding_path));
  try!(rs_binding_stream.write_all(b"#[link(name = \"rustcryptopp\")]\n"));

  let ctx = gen::Context::new(cpp_stream, rs_binding_stream);
  try!(gen_classes(ctx, out_path));

  Ok(())
}
 
fn main() {
  let cryptopp_lib = find_cryptopp();
  println!("cryptopp: {:?}", cryptopp_lib);

  let out_dir = std::env::var("OUT_DIR").unwrap();
  let out_path = std::path::Path::new(&out_dir);
  let rust_binding_src = out_path.join("generated_bindings.rs");
  let cpp_src = out_path.join("generated_cpp.cpp");

  if let Err(e) = gen_cpp_code(&cpp_src, &rust_binding_src, out_path) {
    println!("error: {:?}\n", e);
    return;
  }

  let mut config = gcc::Config::new();
  config.cpp(true);

  config.file(&cpp_src);

  config.compile("librustcryptopp.a");
}
