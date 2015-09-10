use hash;

#[derive(Debug)]
pub struct DigestTest {
  pub algorithm: hash::Algorithm,
  pub msg: Vec<u8>,
  pub digest: Vec<u8>
}

#[derive(Debug)]
pub enum Vector {
  Digest(DigestTest)
}

pub mod cryptopp {
  use std::path::{Path, PathBuf};
  use std::fs::File;
  use std::env;
  use std::iter::Iterator;
  use std::io;
  use std::io::Read;

  use nom::{Consumer,Producer,ConsumerState,FileProducer,IResult};

  use hash;

  #[derive(Debug)]
  enum VectorParser {
    Start,
    ParsingTests(TestHeader)
  }

  pub struct Vector {
    file: FileProducer,
    parser: VectorParser
  }

  #[derive(Debug, Clone, Copy)]
  pub enum TestHeader {
    MessageDigest(hash::Algorithm)
  }

  #[derive(Debug)]
  enum ParseErr {
    UnknownMessageDigestName(String),
    UnknownAlgorithmType(String),
    IO(io::Error),
    EOF
  }

//  #[derive(Debug)]
//  pub struct VectorParser {
//    state: VectorParserState
//  }

  fn parse_header(stream: &mut FileProducer) -> Result<TestHeader, ParseErr> {
    named!(abcd, tag!("abcd") );

    assert_eq!(abcd(stream), IResult::Done(&b"xxx"[..], &b"abcd"[..]));
    unimplemented!()
  }

  fn parse_test(header: &TestHeader, stream: &mut FileProducer)
     -> Result<Option<super::Vector>, ParseErr> {
    unimplemented!()
  }

  impl VectorParser {
    fn new() -> VectorParser {
      self::VectorParser::Start
    }

    fn parse(&self, stream: &mut FileProducer)
       -> Result<(VectorParser, Option<super::Vector>), ParseErr> {
      use self::VectorParser::*;

      let result = match self {
        &Start                => {
          let header = try!(parse_header(stream));
          (ParsingTests(header), None)
        },
        &ParsingTests(header) => {
          let maybe_test = try!(parse_test(&header, stream));
          match maybe_test {
            Some(test) => (ParsingTests(header), Some(test)),
            None       => (Start, None)
          }
        }
      };

      Ok(result)
    }

    fn parse_next_test(&self, stream: &mut FileProducer)
       -> Result<(VectorParser, super::Vector), ParseErr> {
      match self.parse(stream) {
        Ok((state, maybe_tv)) => {
          match maybe_tv {
            Some(tv) => {
              return Ok((state, tv));
            },
            None     => unimplemented!()
          }          
        },
        Err(err)              => Err(err)
      }
    }
  }

  impl Vector {
    fn path(name: &str) -> PathBuf {
      let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
      let manifest_path = Path::new(&manifest_dir);
      let mut pbuf = manifest_path.to_path_buf();
      pbuf.push("test_vectors");
      pbuf.push(name);

      pbuf
    }

    pub fn new(name: &str) -> Vector {
      let path = Vector::path(name);

      let f = FileProducer::new(path.to_str().unwrap(), 1024).unwrap();
      Vector { file:  f,
               parser: VectorParser::new() }
    }

    fn parse_next_test(&mut self) -> Option<Result<super::Vector, ParseErr>> {
      match self.parser.parse_next_test(&mut self.file) {
        Ok((new_parser, tv))     => {
          self.parser = new_parser;
          Some(Ok(tv))
        },
        Err(self::ParseErr::EOF) => None,
        Err(err)                 => Some(Err(err))
      }
    }
  }

  impl Iterator for Vector {
    type Item = super::Vector;

    fn next(&mut self) -> Option<Self::Item> {
      match self.parse_next_test() {
        Some(result) => Some(result.unwrap()),
        None         => None
      }
    }
  }
}
