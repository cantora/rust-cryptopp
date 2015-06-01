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

fn gen_cpp_code(out_path: &std::path::Path) -> io::Result<()> {
  use gen::proto::*;
  use std::io::Write;
  use std::fs::File;

  let mut f_stream = try!(File::create(out_path));
  let namespace = vec![];

  let class = class_methods!(gen::class() => {
    void(), b"Update",     mut_ptr(UChar), size_t();
    void(), b"Final",      mut_ptr(UChar);
    void(), b"Restart";
    uint(), b"DigestSize";
  });

  try!(f_stream.write_all(b"#include <cryptopp/cryptlib.h>\n"));
  try!(f_stream.write_all(b"using namespace CryptoPP;\n\n"));
  class.generate_cpp(&namespace,
                     b"HashTransformation",
                     &mut f_stream)
}
 
fn main() {
  use std::fs;

  let cryptopp_lib = find_cryptopp();
  println!("cryptopp: {:?}", cryptopp_lib);

  let out_dir = std::env::var("OUT_DIR").unwrap();
  let out_path = std::path::Path::new(&out_dir);
  let cpp_src = out_path.join("cryptopp.cpp");

  gen_cpp_code(&cpp_src).unwrap();

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
