
macro_rules! array_impls {
  ($( $tname:ident , $N:expr );+) => {
    $(
      pub struct $tname<T> ([T; $N]);

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

      impl<T: Eq> Eq for $tname<T> { }

      impl<T:PartialOrd> PartialOrd for $tname<T> {
        #[inline]
        fn partial_cmp(&self, other: &[T; $N]) -> Option<Ordering> {
          PartialOrd::partial_cmp(&&self[..], &&other[..])
        }
        #[inline]
        fn lt(&self, other: &[T; $N]) -> bool {
          PartialOrd::lt(&&self[..], &&other[..])
        }
        #[inline]
        fn le(&self, other: &[T; $N]) -> bool {
          PartialOrd::le(&&self[..], &&other[..])
        }
        #[inline]
        fn ge(&self, other: &[T; $N]) -> bool {
          PartialOrd::ge(&&self[..], &&other[..])
        }
        #[inline]
        fn gt(&self, other: &[T; $N]) -> bool {
          PartialOrd::gt(&&self[..], &&other[..])
        }
      }

      impl<T:Ord> Ord for $tname<T> {
        #[inline]
        fn cmp(&self, other: &$tname<T>) -> Ordering {
          Ord::cmp(&&self[..], &&other[..])
        }
      }
    )+
  }
}

//array_impls! {
//  Arr20,20
//}
