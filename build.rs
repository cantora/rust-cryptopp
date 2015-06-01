extern crate pkg_config;
extern crate gcc;

#[macro_use(function, void_function, class_methods)]
extern crate rust_cryptopp_gen as gen;

use std::io;

fn find_cryptopp() -> pkg_config::Library {
  match pkg_config::find_library("libcrypto++") {
    Ok(info) => info,
    Err(err) => panic!("failed to find cryptopp: {}", err)
  }
}

fn gen_cpp_code(cpp_path: &std::path::Path,
                rust_path: &std::path::Path) -> io::Result<()> {
  use gen::proto::*;
  use std::io::Write;
  use std::fs::File;

  let mut cpp_stream = try!(File::create(cpp_path));
  let mut rs_stream = try!(File::create(rust_path));
  let namespace = vec![];

  let class = class_methods!(gen::class() => {
    void(), b"Update",     const_ptr(UChar), size_t();
    void(), b"Final",      mut_ptr(UChar);
    void(), b"Restart";
    uint(), b"DigestSize";
  });

  try!(cpp_stream.write_all(b"#include <cryptopp/cryptlib.h>\n"));
  try!(cpp_stream.write_all(b"using namespace CryptoPP;\n\n"));
  try!(class.generate_cpp(&namespace,
                          b"HashTransformation",
                          &mut cpp_stream));

  try!(rs_stream.write_all(b"#[link(name = \"rustcryptopp\")]\n"));
  try!(class.generate_rs(&namespace,
                         b"HashTransformation",
                         &mut rs_stream));
  Ok(())
}
 
fn main() {
  use std::fs;

  let cryptopp_lib = find_cryptopp();
  println!("cryptopp: {:?}", cryptopp_lib);

  let out_dir = std::env::var("OUT_DIR").unwrap();
  let out_path = std::path::Path::new(&out_dir);
  let rust_src = out_path.join("gen.rs");
  let cpp_src = out_path.join("gen.cpp");

  gen_cpp_code(&cpp_src, &rust_src).unwrap();

  let mut config = gcc::Config::new();
  config.cpp(true);

  config.file(&cpp_src);

  for dent in fs::read_dir("src/cpp").unwrap() {
    let path = dent.unwrap().path();
    let item = fs::metadata(&path).unwrap();
    if !item.is_file() {
      continue;
    }

    if let Some(ref ext) = path.extension() {
      if *ext == "cpp" {
        println!("{:?}", path);
        config.file(&path);
      }
    }
  }

  config.compile("librustcryptopp.a");
}
