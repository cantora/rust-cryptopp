extern crate pkg_config;
extern crate gcc;

fn find_cryptopp() -> pkg_config::Library {
  match pkg_config::find_library("libcrypto++") {
    Ok(info) => info,
    Err(err) => panic!("failed to find cryptopp: {}", err)
  }
}

fn main() {
  use std::fs;

  let cryptopp_lib = find_cryptopp();
  println!("cryptopp: {:?}", cryptopp_lib);

  let mut config = gcc::Config::new();
  config.cpp(true);

  for dent in fs::read_dir("src/cpp").unwrap() {
    let path = dent.unwrap().path();
    println!("{:?}", path);
    let item = fs::metadata(&path).unwrap();
    if item.is_file() {
      config.file(path);
    }
  }

  config.compile("librustcryptopp.a");
}
