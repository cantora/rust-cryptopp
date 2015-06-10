extern crate pkg_config;
extern crate gcc;

#[macro_use]
extern crate rust_cryptopp_gen as gen;

use std::io;
use std::io::Write;
use std::fs::File;

fn find_cryptopp() -> pkg_config::Library {
  match pkg_config::find_library("libcrypto++") {
    Ok(info) => info,
    Err(err) => panic!("failed to find cryptopp: {}", err)
  }
}

fn gen_classes(mut ctx: gen::Context<File, File>) -> io::Result<()> {
  use gen::proto::*;

  try!(ctx.generate(class!(b"HashTransformation" => {
    mutable methods {
      void(), b"Update",     const_ptr(UChar), size_t();
      void(), b"Final",      mut_ptr(UChar);
      void(), b"Restart";
    }
    constant methods {
      uint(), b"DigestSize";
    }
  })));

  try!(ctx.generate(class!(b"SHA3_256" => {
    constructors {
      b"";
    }
  })));

  ctx.generate(class!(b"Integer" => {
    constructors {
      b"";
      b"copy",      const_ref(Custom(b"Integer"));
      b"from_long", long();
    }
  }))
}

fn gen_cpp_code(cpp_path: &std::path::Path,
                rust_path: &std::path::Path) -> io::Result<()> {

  let mut cpp_stream = try!(File::create(cpp_path));

  try!(cpp_stream.write_all(b"#include <cryptopp/cryptlib.h>\n"));
  try!(cpp_stream.write_all(b"#include <cryptopp/sha3.h>\n"));
  try!(cpp_stream.write_all(b"#include <cryptopp/integer.h>\n"));
  try!(cpp_stream.write_all(b"using namespace CryptoPP;\n\n"));

  let mut rs_stream = try!(File::create(rust_path));
  try!(rs_stream.write_all(b"#[link(name = \"rustcryptopp\")]\n"));

  let ctx = gen::Context::new(cpp_stream, rs_stream);
  gen_classes(ctx)
}
 
fn main() {
  let cryptopp_lib = find_cryptopp();
  println!("cryptopp: {:?}", cryptopp_lib);

  let out_dir = std::env::var("OUT_DIR").unwrap();
  let out_path = std::path::Path::new(&out_dir);
  let rust_src = out_path.join("generated_bindings.rs");
  let cpp_src = out_path.join("generated_cpp.cpp");

  gen_cpp_code(&cpp_src, &rust_src).unwrap();

  let mut config = gcc::Config::new();
  config.cpp(true);

  config.file(&cpp_src);

  config.compile("librustcryptopp.a");
}
