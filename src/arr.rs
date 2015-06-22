
use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
use std::fmt;
use std::hash::{Hash, self};
use std::iter::IntoIterator;
use std::slice::{Iter, IterMut};
use std::default::Default;

macro_rules! array_wrap_base {
  ($tname:ident , $titem:ty , $N:expr) => (
    pub struct $tname ([$titem; $N]);

    impl $tname {
      pub fn from_array(array: [$titem; $N]) -> $tname {
        $tname(array)
      }

      pub fn into_array(self) -> [$titem; $N] {
        self.0
      }

      pub fn array(&self) -> &[$titem; $N] {
        &self.0
      }

      pub fn mut_array(&mut self) -> &mut [$titem; $N] {
        &mut self.0
      }
    }
  )
}

macro_rules! array_wrap_trait_Default {
  ($tname:ident , $titem:ty , $N:expr) => (
    impl Default for $tname {
      fn default() -> $tname {
        $tname([<$titem as Default>::default(); $N])
      }
    }
  )
}

macro_rules! array_wrap_trait_From_array {
  ($tname:ident , $titem:ty , $N:expr) => (
    impl From<[$titem; $N]> for $tname {
      fn from(arr: [$titem; $N]) -> $tname {
        $tname(arr)
      }
    }
  )
}

macro_rules! array_wrap_trait_Into_array {
  ($tname:ident , $titem:ty , $N:expr) => (
    impl Into<[$titem; $N]> for $tname {
      fn into(self) -> [$titem; $N] {
        self.into_array()
      }
    }
  )
}

macro_rules! array_wrap_trait_AsRef_array {
  ($tname:ident , $titem:ty , $N:expr) => (
    impl AsRef<[$titem; $N]> for $tname {
      #[inline]
      fn as_ref(&self) -> &[$titem; $N] {
        self.array()
      }
    }
  )
}

macro_rules! array_wrap_trait_AsMut_array {
  ($tname:ident , $titem:ty , $N:expr) => (
    impl AsMut<[$titem; $N]> for $tname {
      #[inline]
      fn as_mut(&mut self) -> &mut [$titem; $N] {
        self.mut_array()
      }
    }
  )
}

macro_rules! array_wrap_trait_AsRef_slice {
  ($tname:ident , $titem:ty ) => (
    impl AsRef<[$titem]> for $tname {
      #[inline]
      fn as_ref(&self) -> &[$titem] {
        &self.0[..]
      }
    }
  )
}

macro_rules! array_wrap_trait_AsMut_slice {
  ($tname:ident , $titem:ty ) => (
    impl AsMut<[$titem]> for $tname {
      #[inline]
      fn as_mut(&mut self) -> &mut [$titem] {
        &mut self.0[..]
      }
    }
  )
}

macro_rules! array_wrap_trait_Hash {
  ($tname:ident ) => (
    impl hash::Hash for $tname {
      fn hash<H: hash::Hasher>(&self, state: &mut H) {
        hash::Hash::hash(&self.0[..], state)
      }
    }
  )
}

macro_rules! array_wrap_trait_Debug {
  ($tname:ident ) => (
    impl fmt::Debug for $tname {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&&self.0[..], f)
      }
    }
  )
}

macro_rules! array_wrap_trait_IntoIterator {
  ($tname:ident , $titem:ty ) => (
    impl<'a> IntoIterator for &'a $tname {
      type Item = &'a $titem;
      type IntoIter = slice::Iter<'a, $titem>;

      fn into_iter(self) -> slice::Iter<'a, $titem> {
        self.0[..].iter()
      }
    }
  )
}

macro_rules! array_wrap_trait_IntoIterator_mut {
  ($tname:ident , $titem:ty ) => (
    impl<'a> IntoIterator for &'a mut $tname {
      type Item = &'a mut $titem;
      type IntoIter = slice::IterMut<'a, $titem>;

      fn into_iter(self) -> slice::IterMut<'a, $titem> {
        self.0[..].iter_mut()
      }
    }
  )
}

macro_rules! array_wrap_trait_PartialEq {
  ($tname:ident ) => (
    impl PartialEq for $tname {
      #[inline]
      fn eq(&self, other: &Self) -> bool { &self.0[..] == &other.0[..] }
      #[inline]
      fn ne(&self, other: &Self) -> bool { &self.0[..] != &other.0[..] }
    }
  )
}

macro_rules! array_wrap_trait_Eq {
  ($tname:ident ) => (
    impl Eq for $tname { }
  )
}

macro_rules! array_wrap_trait_PartialOrd {
  ($tname:ident ) => (
    impl PartialOrd for $tname {
      #[inline]
      fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&&self.0[..], &&other.0[..])
      }
      #[inline]
      fn lt(&self, other: &Self) -> bool {
        PartialOrd::lt(&&self.0[..], &&other.0[..])
      }
      #[inline]
      fn le(&self, other: &Self) -> bool {
        PartialOrd::le(&&self.0[..], &&other.0[..])
      }
      #[inline]
      fn ge(&self, other: &Self) -> bool {
        PartialOrd::ge(&&self.0[..], &&other.0[..])
      }
      #[inline]
      fn gt(&self, other: &Self) -> bool {
        PartialOrd::gt(&&self.0[..], &&other.0[..])
      }
    }
  )
}

macro_rules! array_wrap_trait_Ord {
  ($tname:ident) => (
    impl Ord for $tname {
      #[inline]
      fn cmp(&self, other: &$tname) -> cmp::Ordering {
        Ord::cmp(&&self.0[..], &&other.0[..])
      }
    }
  )
}

macro_rules! array_wrap_traits {
  ($tname:ident , $titem:ty , $N:expr , ) => ();

  ($tname:ident , $titem:ty , $N:expr, Default $( $rest:tt )*) => (
    array_wrap_trait_Default!($tname , $titem , $N);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, From_array $( $rest:tt )*) => (
    array_wrap_trait_From_array!($tname , $titem , $N);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, Into_array $( $rest:tt )*) => (
    array_wrap_trait_Into_array!($tname , $titem , $N);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, AsRef_array $( $rest:tt )*) => (
    array_wrap_trait_AsRef_array!($tname , $titem , $N);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, AsMut_array $( $rest:tt )*) => (
    array_wrap_trait_AsMut_array!($tname , $titem , $N);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, AsRef_slice $( $rest:tt )*) => (
    array_wrap_trait_AsRef_slice!($tname , $titem);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, AsMut_slice $( $rest:tt )*) => (
    array_wrap_trait_AsMut_slice!($tname , $titem);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, Hash $( $rest:tt )*) => (
    array_wrap_trait_Hash!($tname);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, Debug $( $rest:tt )*) => (
    array_wrap_trait_Debug!($tname);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, IntoIterator $( $rest:tt )*) => (
    array_wrap_trait_IntoIterator!($tname, $titem);
    array_wrap_trait_IntoIterator_mut!($tname, $titem);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, PartialEq $( $rest:tt )*) => (
    array_wrap_trait_PartialEq!($tname);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, Eq $( $rest:tt )*) => (
    array_wrap_trait_Eq!($tname);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, PartialOrd $( $rest:tt )*) => (
    array_wrap_trait_PartialOrd!($tname);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );

  ($tname:ident , $titem:ty , $N:expr, Ord $( $rest:tt )*) => (
    array_wrap_trait_Ord!($tname);
    array_wrap_traits!($tname , $titem , $N, $( $rest )*);
  );
}

macro_rules! array_wrap {
  ($tname:ident , $titem:ty , $N:expr, $( $rest:tt )+) => (
    array_wrap_base!($tname , $titem , $N);
    array_wrap_traits!($tname , $titem , $N, $( $rest )+);
  )
}

macro_rules! array_impls {
  ($( $tname:ident , $N:expr );+) => {
    $(
     pub struct $tname<T> ([T; $N]);

     impl<T> $tname<T> {
        pub fn from_array(array: [T; $N]) -> $tname<T> {
          $tname(array)
        }

        pub fn array(&self) -> &[T; $N] {
          self.as_ref()
        }

        pub fn mut_array(&mut self) -> &mut [T; $N] {
          self.as_mut()
        }
      }

      impl<T:Default + Copy> Default for $tname<T> {
        fn default() -> $tname<T> {
          $tname([T::default(); $N])
        }
      }

      impl<T> From<[T; $N]> for $tname<T> {
        fn from(arr: [T; $N]) -> $tname<T> {
          $tname(arr)
        }
      }

      impl<T> Into<[T; $N]> for $tname<T> {
        fn into(self) -> [T; $N] {
          self.0
        }
      }

      impl<T> AsRef<[T; $N]> for $tname<T> {
        #[inline]
        fn as_ref(&self) -> &[T; $N] {
          &self.0
        }
      }

      impl<T> AsMut<[T; $N]> for $tname<T> {
        #[inline]
        fn as_mut(&mut self) -> &mut [T; $N] {
          &mut self.0
        }
      }

      impl<T> AsRef<[T]> for $tname<T> {
        #[inline]
        fn as_ref(&self) -> &[T] {
          &self.0[..]
        }
      }

      impl<T> AsMut<[T]> for $tname<T> {
        #[inline]
        fn as_mut(&mut self) -> &mut [T] {
          &mut self.0[..]
        }
      }

      impl<T: Hash> Hash for $tname<T> {
        fn hash<H: hash::Hasher>(&self, state: &mut H) {
          Hash::hash(&self.0[..], state)
        }
      }

      impl<T: fmt::Debug> fmt::Debug for $tname<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          fmt::Debug::fmt(&&self.0[..], f)
        }
      }

      impl<'a, T> IntoIterator for &'a $tname<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Iter<'a, T> {
          self.0[..].iter()
        }
      }

      impl<'a, T> IntoIterator for &'a mut $tname<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;

        fn into_iter(self) -> IterMut<'a, T> {
          self.0[..].iter_mut()
        }
      }

      impl<T: PartialEq> PartialEq for $tname<T> {
        #[inline]
        fn eq(&self, other: &Self) -> bool { &self.0[..] == &other.0[..] }
        #[inline]
        fn ne(&self, other: &Self) -> bool { &self.0[..] != &other.0[..] }
      }

      impl<T: Eq> Eq for $tname<T> { }

      impl<T:PartialOrd> PartialOrd for $tname<T> {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
          PartialOrd::partial_cmp(&&self.0[..], &&other.0[..])
        }
        #[inline]
        fn lt(&self, other: &Self) -> bool {
          PartialOrd::lt(&&self.0[..], &&other.0[..])
        }
        #[inline]
        fn le(&self, other: &Self) -> bool {
          PartialOrd::le(&&self.0[..], &&other.0[..])
        }
        #[inline]
        fn ge(&self, other: &Self) -> bool {
          PartialOrd::ge(&&self.0[..], &&other.0[..])
        }
        #[inline]
        fn gt(&self, other: &Self) -> bool {
          PartialOrd::gt(&&self.0[..], &&other.0[..])
        }
      }

      impl<T:Ord> Ord for $tname<T> {
        #[inline]
        fn cmp(&self, other: &$tname<T>) -> Ordering {
          Ord::cmp(&&self.0[..], &&other.0[..])
        }
      }
    )+
  }
}

array_impls! {
  Arr28, 28;
  Arr32, 32;
  Arr48, 48;
  Arr64, 64
}
