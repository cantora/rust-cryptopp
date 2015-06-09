use std::collections::hash_map::HashMap;
use std::io;
use std::io::Write;

pub struct Context<T, U> {
  cpp_stream: T,
  rs_stream: U
}

impl<T: Write, U: Write> Context<T, U> {
  pub fn new(cpp_stream: T, rs_stream: U) -> Context<T, U> {
    Context { cpp_stream: cpp_stream, rs_stream: rs_stream }
  }

  pub fn generate<'a>(&mut self, class: NamedClass<'a>) -> io::Result<()> {
    class.anon_class.generate(&class.namespace,
                              class.name,
                              &mut self.cpp_stream,
                              &mut self.rs_stream)
  }
}

pub enum FunctionArgs {
  None,
  Args1([proto::BasicType; 1]),
  Args2([proto::BasicType; 2]),
  Args3([proto::BasicType; 3]),
}

impl FunctionArgs {
  pub fn as_slice(&self) -> Option<&[proto::BasicType]> {
    use self::FunctionArgs::*;

    macro_rules! args_arm {
      ($arr:expr) => (
        Some(&$arr[..])
      )
    }

    match self {
      &None           => Option::None,
      &Args1(ref arr) => args_arm!(arr),
      &Args2(ref arr) => args_arm!(arr),
      &Args3(ref arr) => args_arm!(arr),
    }
  }

  pub fn len(&self) -> usize {
    match self.as_slice() {
      Some(ref slice) => slice.len(),
      None            => 0
    }
  }

  pub fn generate_cpp(&self, out_stream: &mut Write) -> io::Result<()> {
    if let Some(slice) = self.as_slice() {
      let mut i = 0u32;

      for btype in slice.iter() {
        if i > 0 {
          try!(out_stream.write_all(b", "));
        }

        try!(btype.generate_cpp(out_stream));

        try!(write!(out_stream, " arg{}", i));
        i += 1;
      }
    }

    Ok(())
  }

  pub fn generate_rs(&self, out_stream: &mut Write) -> io::Result<()> {
    if let Some(slice) = self.as_slice() {
      let mut i = 0u32;

      for btype in slice.iter() {
        if i > 0 {
          try!(out_stream.write_all(b", "));
        }

        try!(write!(out_stream, "arg{}: ", i));
        try!(btype.generate_rs(out_stream));

        i += 1;
      }
    }

    Ok(())
  }
}

pub struct Function {
  pub ret: proto::BasicType,
  pub args: FunctionArgs
}

pub struct Method {
  func: Function,
  is_const: bool
}

pub fn method(func: Function, is_const: bool) -> Method {
  Method {
    func: func,
    is_const: is_const
  }
}

#[macro_export]
macro_rules! class {
  ( $name:expr => $b:tt ) => ({
    let anon_class = class_bindings_block!($crate::class() => $b);
    $crate::NamedClass::new(
      vec![],
      $name,
      anon_class
    )
  });

  ( $ns:expr, $name:expr => $b:tt ) => ({
    let anon_class = class_bindings_block!($crate::class() => $b);
    $crate::NamedClass::new(
      $ns,
      $name,
      anon_class
    )
  });
}

pub struct NamedClass<'a> {
  pub namespace:  Vec<&'a [u8]>,
  pub name:       &'a [u8],
  pub anon_class: Class
}

impl<'a> NamedClass<'a> {
  pub fn new(namespace:  Vec<&'a [u8]>,
             name:       &'a [u8],
             anon_class: Class) -> NamedClass<'a> {
    NamedClass {
      namespace: namespace,
      name: name,
      anon_class: anon_class
    }
  }
}

//pub enum BindingType {
//  MutableMethod,
//  ConstMethod,
//}
//
//pub struct BindingCtx {
//  pub cls: Class,
//  pub binding_type: 
//}

#[macro_export]
macro_rules! class_bindings_block {
  ($cls:expr => { $( $t:tt )* } ) => ({
    let mut cls = $cls;
    class_bindings!(cls, $( $t )*)
  })
}

#[macro_export]
macro_rules! class_bindings {
  ($cls:expr, ) => (
    $cls
  );

  ($cls:expr, mutable methods { $( $t:tt )* } $( $rest:tt )* ) => ({
    let cls = class_methods!($cls, false, $( $t )*);
    class_bindings!(cls, $( $rest )* )
  });

  ($cls:expr, constant methods { $( $t:tt )* } $( $rest:tt )* ) => ({
    let cls = class_methods!($cls, true, $( $rest )*);
    class_bindings!(cls, $( $rest )* )
  });

  ($cls:expr, constructors { $( $t:tt )* } $( $rest:tt )* ) => ({
    let cls = class_ctors!($cls, $( $t )*);
    class_bindings!(cls, $( $rest )* )
  });

}

#[macro_export]
macro_rules! class_methods {
  ($cls:expr , $is_const:expr , ) => (
    $cls
  );

  ($cls:expr , $is_const:expr , $rtype:expr , $mname:expr ; $( $rest:tt )* ) => ({
    $cls.add_method($mname, $is_const, function!($rtype) );
    class_methods!($cls, $is_const, $( $rest )* )
  });

  ($cls:expr , $is_const:expr , $rtype:expr , $mname:expr, $( $args:expr ),+ ; $( $rest:tt )* ) => ({
    $cls.add_method($mname, $is_const, function!($rtype, $( $args ),+ ) );
    class_methods!($cls, $is_const, $( $rest )* )
  });
}

#[macro_export]
macro_rules! class_ctors {
  ($cls:expr , ) => (
    $cls
  );

  ($cls:expr , $mname:expr ; $( $rest:tt )* ) => ({
    $cls.add_constructor($mname, function_args!() );
    class_ctors!($cls, $( $rest )* )
  });

  ($cls:expr , $mname:expr , $( $args:expr ),+ ; $( $rest:tt )* ) => ({
    $cls.add_constructor($mname, function_args!($( $args ),+ ) );
    class_ctors!($cls, $( $rest )* )
  });
}

pub struct Class {
  methods: HashMap<&'static [u8], Method>,
  ctors: HashMap<&'static [u8], FunctionArgs>
}

pub fn class() -> Class {
  Class {
    methods: HashMap::new(),
    ctors:   HashMap::new()
  }
}

fn generate_c_path(namespace: &Vec<&[u8]>,
                   name: &[u8],
                   out: &mut Write) -> io::Result<()> {
  for ns_part in namespace.iter() {
    try!(out.write_all(ns_part));
    try!(out.write_all(b"_"));
  }
  out.write_all(name)
}

fn generate_cpp_path(namespace: &Vec<&[u8]>,
                    name: &[u8],
                    out: &mut Write) -> io::Result<()> {
  for ns_part in namespace.iter() {
    try!(out.write_all(ns_part));
    try!(out.write_all(b"::"));
  }
  out.write_all(name)
}

impl Class {
  pub fn add_method(&mut self,
                    name: &'static [u8],
                    is_const: bool,
                    function: Function) {
    self.methods.insert(name, method(function, is_const));
  }

  pub fn add_constructor(&mut self, name: &'static [u8], args: FunctionArgs) {
    self.ctors.insert(name, args);
  }

  pub fn generate_cpp(&self,
                      namespace: &Vec<&[u8]>,
                      name: &[u8],
                      out_stream: &mut Write) -> io::Result<()> {
    try!(self.generate_cpp_ctors(namespace, name, out_stream));
    self.generate_cpp_methods(namespace, name, out_stream)
  }

  fn generate_cpp_ctors(&self,
                        namespace: &Vec<&[u8]>,
                        name: &[u8],
                        out_stream: &mut Write) -> io::Result<()> {
    for (ctor_name, ctor_args) in self.ctors.iter() {
      try!(out_stream.write_all(b"extern \"C\"\n"));

      try!(generate_cpp_path(namespace, name, out_stream));
      try!(out_stream.write_all(b" *"));

      try!(out_stream.write_all(b" new_"));
      if ctor_name.len() > 0 {
        try!(out_stream.write_all(ctor_name));
        try!(out_stream.write_all(b"_"));
      }
      try!(generate_c_path(namespace, name, out_stream));

      try!(out_stream.write_all(b"("));

      let arg_len = ctor_args.len();
      if arg_len > 0 {
        try!(ctor_args.generate_cpp(out_stream));
      }

      try!(out_stream.write_all(b")"));

      try!(out_stream.write_all(b" {\n  "));
      try!(out_stream.write_all(b"return new "));

      try!(generate_cpp_path(namespace, name, out_stream));

      try!(out_stream.write_all(b"("));

      for i in (0..arg_len) {
        if i > 0 {
          try!(out_stream.write_all(b", "));
        }
        try!(write!(out_stream, "arg{}", i));
      }

      try!(out_stream.write_all(b");\n"));
      try!(out_stream.write_all(b"}"));
      try!(out_stream.write_all(b"\n\n"));
    }

    try!(out_stream.write_all(b"extern \"C\"\n"));
    try!(out_stream.write_all(b"void del_"));

    try!(generate_c_path(namespace, name, out_stream));
    try!(out_stream.write_all(b"("));

    try!(generate_cpp_path(namespace, name, out_stream));
    try!(out_stream.write_all(b"*"));
    try!(out_stream.write_all(b" ctx"));
    try!(out_stream.write_all(b")"));
    try!(out_stream.write_all(b" {\n  delete ctx;\n}\n\n"));

    Ok(())
  }

  fn generate_cpp_methods(&self,
                          namespace: &Vec<&[u8]>,
                          name: &[u8],
                          out_stream: &mut Write) -> io::Result<()> {
    for (method_name, method_desc) in self.methods.iter() {
      let function_desc = &method_desc.func;

      try!(out_stream.write_all(b"extern \"C\"\n"));

      try!(function_desc.ret.generate_cpp(out_stream));

      try!(out_stream.write_all(b" mth_"));
      try!(generate_c_path(namespace, name, out_stream));
      try!(out_stream.write_all(b"_"));
      try!(out_stream.write_all(method_name));
      try!(out_stream.write_all(b"("));

      try!(generate_cpp_path(namespace, name, out_stream));
      if method_desc.is_const {
        try!(out_stream.write_all(b" const"));
      }
      try!(out_stream.write_all(b"*"));
      try!(out_stream.write_all(b" ctx"));

      let arg_len = function_desc.args.len();
      if arg_len > 0 {
        try!(out_stream.write_all(b", "));
        try!(function_desc.args.generate_cpp(out_stream));
      }
      try!(out_stream.write_all(b")"));

      try!(out_stream.write_all(b" {\n  "));
      if !function_desc.ret.is_void() {
        try!(out_stream.write_all(b"return "));
      }
      try!(out_stream.write_all(b"ctx->"));
      try!(out_stream.write_all(method_name));
      try!(out_stream.write_all(b"("));

      for i in (0..arg_len) {
        if i > 0 {
          try!(out_stream.write_all(b", "));
        }
        try!(write!(out_stream, "arg{}", i));
      }

      try!(out_stream.write_all(b");\n"));
      try!(out_stream.write_all(b"}"));
      try!(out_stream.write_all(b"\n\n"));
    }

    Ok(())
  }

  pub fn generate_rs(&self,
                     namespace: &Vec<&[u8]>,
                     name: &[u8],
                     out_stream: &mut Write) -> io::Result<()> {
    try!(out_stream.write_all(
      b"extern {\n"
    ));

    try!(self.generate_rs_methods(namespace, name, out_stream));
    try!(self.generate_rs_ctors(namespace, name, out_stream));

    try!(out_stream.write_all(b"}\n"));

    Ok(())
  }

  fn generate_rs_ctors(&self,
                       namespace: &Vec<&[u8]>,
                       name: &[u8],
                       out_stream: &mut Write) -> io::Result<()> {
    if self.ctors.len() < 1 {
      return Ok(())
    }

    for (ctor_name, ctor_args) in self.ctors.iter() {
      try!(out_stream.write_all(b"  pub fn new_"));
      if ctor_name.len() > 0 {
        try!(out_stream.write_all(ctor_name));
        try!(out_stream.write_all(b"_"));
      }
      try!(generate_c_path(namespace, name, out_stream));

      try!(out_stream.write_all(b"("));

      let arg_len = ctor_args.len();
      if arg_len > 0 {
        try!(ctor_args.generate_rs(out_stream));
      }

      try!(out_stream.write_all(b")"));
      try!(out_stream.write_all(b" -> *mut c_void"));
      try!(out_stream.write_all(b";\n"));
    }

    try!(out_stream.write_all(b"  pub fn del_"));
    try!(generate_c_path(namespace, name, out_stream));
    try!(out_stream.write_all(b"(ctx: *mut c_void);\n"));

    Ok(())
  }

  fn generate_rs_methods(&self,
                         namespace: &Vec<&[u8]>,
                         name: &[u8],
                         out_stream: &mut Write) -> io::Result<()> {
    for (method_name, method_desc) in self.methods.iter() {
      let function_desc = &method_desc.func;

      try!(out_stream.write_all(b"  pub fn mth_"));

      try!(generate_c_path(namespace, name, out_stream));
      try!(out_stream.write_all(b"_"));
      try!(out_stream.write_all(method_name));
      try!(out_stream.write_all(b"("));

      try!(out_stream.write_all(b"ctx: *"));
      try!(out_stream.write_all(if method_desc.is_const {
        b"const "
      } else {
        b"mut "
      }));
      try!(out_stream.write_all(b"c_void"));

      let arg_len = function_desc.args.len();
      if arg_len > 0 {
        try!(out_stream.write_all(b", "));
        try!(function_desc.args.generate_rs(out_stream));
      }

      try!(out_stream.write_all(b")"));

      if !function_desc.ret.is_void() {
        try!(out_stream.write_all(b" -> "));
        try!(function_desc.ret.generate_rs(out_stream));
      }
      try!(out_stream.write_all(b";\n"));
    }

    Ok(())
  }

  pub fn generate(&self,
              namespace: &Vec<&[u8]>,
              name: &[u8],
              cpp_stream: &mut Write,
              rs_stream: &mut Write) -> io::Result<()> {
    try!(self.generate_cpp(&namespace, name, cpp_stream));
    self.generate_rs(&namespace, name, rs_stream)
  }
}

#[macro_export]
macro_rules! function_args {
  () => (
    $crate::FunctionArgs::None
  );

  ($arg1:expr) => (
    $crate::FunctionArgs::Args1([$arg1])
  );

  ($arg1:expr, $arg2:expr) => (
    $crate::FunctionArgs::Args2([$arg1, $arg2])
  );
}

#[macro_export]
macro_rules! function {
  ($rtype:expr) => ($crate::Function {
    ret: $rtype,
    args: function_args!()
  });

  ($rtype:expr, $( $args:expr ),+ ) => ($crate::Function {
    ret: $rtype,
    args: function_args!($( $args ),+)
  });
}

#[macro_export]
macro_rules! void_function {
  ( $ ( $ arg : expr ),* ) => (
    function!($crate::proto::void() $ (, $ arg ) * )
  );
}

pub mod proto {
  use std::io;
  use std::io::Write;

  pub enum BasicType {
    Simple(CType),
    MutPointer(CType),
    ConstPointer(CType)
  }

  impl BasicType {

    pub fn is_void(&self) -> bool {
      if let &BasicType::Simple(CType::Void) = self {
        return true;
      }

      false
    }

    pub fn generate_cpp(&self, out_stream: &mut Write) -> io::Result<()> {
      use self::BasicType::*;

      match self {
        &Simple(ref t)       => t.generate_cpp(out_stream),
        &MutPointer(ref t)   => {
          try!(t.generate_cpp(out_stream));
          out_stream.write_all(b"*")
        },
        &ConstPointer(ref t) => {
          try!(t.generate_cpp(out_stream));
          out_stream.write_all(b" const*")
        }
      } // match self
    } // generate_cpp

    pub fn generate_rs(&self, out_stream: &mut Write) -> io::Result<()> {
      use self::BasicType::*;

      match self {
        &Simple(ref t)       => t.generate_rs(out_stream),
        &MutPointer(ref t)   => {
          try!(out_stream.write_all(b"*mut "));
          t.generate_rs(out_stream)
        },
        &ConstPointer(ref t) => {
          try!(out_stream.write_all(b"*const "));
          t.generate_rs(out_stream)
        }
      } // match self
    } // generate_rs
  }
  
  pub enum CType {
    Void,
    UChar,
    UInt,
    SizeT,
    Custom(&'static [u8])
  }

  pub use self::CType::*;

  impl CType {
    pub fn generate_cpp(&self, out_stream: &mut Write) -> io::Result<()> {
      use self::CType::*;

      out_stream.write_all(match self {
        &Void       => b"void",
        &UChar      => b"unsigned char",
        &SizeT      => b"size_t",
        &UInt       => b"unsigned int",
        &Custom(ref s) => s,
      }) // write_all(match...)
    } // generate_cpp

    pub fn generate_rs(&self, out_stream: &mut Write) -> io::Result<()> {
      use self::CType::*;

      out_stream.write_all(match self {
        &Void       => b"c_void",
        &UChar      => b"c_uchar",
        &SizeT      => b"size_t",
        &UInt       => b"c_uint",
        &Custom(_)  => b"c_void",
      }) // write_all(match...)
    } // generate_rs
  }

  pub fn void() -> BasicType {
    BasicType::Simple(CType::Void)
  }

  pub fn size_t() -> BasicType {
    BasicType::Simple(CType::SizeT)
  }

  pub fn uint() -> BasicType {
    BasicType::Simple(CType::UInt)
  }

  pub fn mut_ptr(t: CType) -> BasicType {
    BasicType::MutPointer(t)
  }

  pub fn const_ptr(t: CType) -> BasicType {
    BasicType::ConstPointer(t)
  }
}
//#define RCPP_NEW(rcpp_t) \
//  extern "C" \
//  rcpp_t * new_ ## rcpp_t () { \
//    return new rcpp_t (); \
//  }
//
