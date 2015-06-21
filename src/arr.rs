
use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
use std::fmt;
use std::hash::{Hash, self};
use std::iter::IntoIterator;
use std::slice::{Iter, IterMut};
use std::default::Default;

#[macro_export]
macro_rules! arr {
  ($t:ty, 28) => (Arr28<$t>);
  ($t:ty, 32) => (Arr32<$t>);
  ($t:ty, 48) => (Arr48<$t>);
  ($t:ty, 64) => (Arr64<$t>);
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