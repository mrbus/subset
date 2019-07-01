//! Multi-subset of slice's items that is able to iterate forward and backward over references to selected items.
//! Each item of a slice can be selected more than once.
//! 
//! In comparison with unique subset, there is no iterator over mutable references (IterMut).
//! The presence of such an iterator would violate (as shown below) Rust's rule that a mutable reference to any ​​memory location must be unique:
//! ```rust,ignore
//! use subset::multi::*;
//! 
//! let mut set = [1, 2];
//! let idxs = [0, 0]; // First item selected twice
//! let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
//! let mut iter = subset.iter_mut();
//! let ref1 = iter.next().unwrap();
//! let ref2 = iter.next().unwrap();
//! assert!(std::ptr::eq(r1, r2));  // Two mut refs to the same memory!
//! *ref1 = 666;
//! assert_eq!(*ref2, 666);
//! ```
//! 
//! # Examples
//!
//! ```
//! use subset::multi::*;
//! 
//! // Constructing mutable multi-subset
//! let mut set = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
//! let idxs = vec![10];
//! assert_eq!(SubsetMut::new(&mut set, &idxs).err(), Some(SubsetError::OutOfBounds));
//! let idxs = vec![2, 2, 5];   // Indexes of selected items
//! let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
//! 
//! // Iterating
//! let mut iter = subset.iter();
//! assert_eq!(Some(&7), iter.next());
//! assert_eq!(Some(&4), iter.next_back());
//! assert_eq!(Some(&7), iter.next_back());
//! assert_eq!(None, iter.next());
//! assert_eq!(None, iter.next_back());
//! 
//! // Converting to immutable multi-subset
//! let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
//! let imm_subset: Subset<_> = subset.into();
//! 
//! // Converting to (unique) mutable subset
//! let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
//! let result: Result<subset::unique::SubsetMut<_>, _> = subset.try_into();
//! assert_eq!(Some(SubsetError::NotUnique), result.err());
//! 
//! // Converting to (unique) immutable subset
//! let idxs = vec![2, 5];   // Indexes of selected items
//! let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
//! let u_imm_subset: subset::unique::Subset<_> = subset.try_into().unwrap();
//! ```

pub use std::convert::{From,Into,TryFrom,TryInto};
use super::{is_unique, unique};
pub use super::SubsetError;


/// Multi-subset of slice's items that is able to iterate forward and backward over references to selected items.
/// Each item of a slice can be selected more than once.
/// 
/// The only difference between Subset and SubsetMut is that Subset holds immutable reference to original set.
#[derive(Debug)]
pub struct Subset<'a, T> {
    pub(crate) set: &'a [T],
    pub(crate) idxs: &'a [usize]
}


/// Double-ended iterator over immutable references to selected items of set.
pub struct Iter<'a, T> {
    ptr: *const T,    // Points to the set
    iter: std::slice::Iter<'a, usize>
}


impl<'a, T> Subset<'a, T> {
    /// Constructs a multi-subset from the whole set and indexes of the selected items.
    /// Array bounds is checked.
    /// Note that subsets are not designed for ZSTs.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use subset::multi::*;
    /// let set = [1, 2, 3];
    /// let idxs = [0, 1];
    /// let subset = Subset::new(&set, &idxs).unwrap();
    /// ```
    /// 
    /// # Errors
    /// OutOfBounds, if any index is `>= set.len()`
    /// 
    /// # Panics
    /// Panics if `std::mem::size_of::<T>() == 0`
    pub fn new(set: &'a [T], idxs: &'a [usize]) -> Result<Self, SubsetError> {
        assert_ne!(std::mem::size_of::<T>(), 0);
        let set_size = set.len();
        if idxs.iter().any(|v| *v >= set_size) {
            Err(SubsetError::OutOfBounds)
        } else { Ok(unsafe{Self::new_unchecked(set, idxs)}) }
    }
    /// Constructs a multi-subset from the whole set and indexes of the selected items.
    /// No array bounds check.
    pub unsafe fn new_unchecked(set: &'a [T], idxs: &'a [usize]) -> Self {
        Self {
            set: set,
            idxs: idxs
        }
    }
    /// Returns the original slice.
    pub fn set(&self) -> &[T] {
        self.set
    }
    /// Returns indexes of selected items.
    pub fn idxs(&self) -> &[usize] {
        self.idxs
    }
    /// Checks that no items are selected twice or more.
    /// if `is_unique() == true` then subset can be converted to unique::Subset.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use subset::multi::*;
    /// 
    /// let set = ["one", "two", "three"];
    /// let idxs = [0, 1];
    /// let subset = Subset::new(&set, &idxs).unwrap();
    /// if subset.is_unique() {
    ///     let uniq_subset: subset::unique::Subset<_> = subset.try_into().unwrap();
    /// }
    /// ```
    pub fn is_unique(&self) -> bool {
        is_unique(self.idxs)
    }
    /// Converts to `subset::unique::Subset`.
    /// Uniqueness of indexes is not checked.
    pub unsafe fn to_unique_unchecked(self) -> unique::Subset<'a, T> {
        unique::Subset {
            m: self
        }
    }
    /// Returns an iterator over immutable references to selected items.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            ptr: self.set.as_ptr(),
            iter: self.idxs.iter()
        }
    }
}


impl<'a, T> From<SubsetMut<'a, T>> for Subset<'a, T> {
    fn from(s: SubsetMut<'a, T>) -> Self {
        Self {
            set: s.set,
            idxs: s.idxs
        }
    }
}


impl<'a, T> From<unique::Subset<'a, T>> for Subset<'a, T> {
    fn from(s: unique::Subset<T>) -> Subset<T> {
        s.m
    }
}


impl<'a, T> From<unique::SubsetMut<'a, T>> for Subset<'a, T> {
    fn from(s: unique::SubsetMut<T>) -> Subset<T> {
        s.m.into()
    }
}


impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            match self.iter.next() {
                None => None,
                Some(idx) => Some(& *self.ptr.offset(*idx as isize))
            }
        }
    }
}


impl<'a, T: 'a> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        unsafe {
            match self.iter.next_back() {
                None => None,
                Some(idx) => Some(& *self.ptr.offset(*idx as isize))
            }
        }
    }
}


/// Multi-subset of slice's items that is able to iterate forward and backward over references to selected items.
/// Each item of a slice can be selected more than once.
/// 
/// The only difference between Subset and SubsetMut is that SubsetMut holds mutable reference to original set.
#[derive(Debug)]
pub struct SubsetMut<'a, T> {
    pub(crate) set: &'a mut [T],
    pub(crate) idxs: &'a [usize]
}

impl<'a, T> SubsetMut<'a, T> {
    /// Constructs a multi-subset from the whole set and indexes of the selected items.
    /// Array bounds is checked.
    /// Note that subsets are not designed for ZSTs.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use subset::multi::*;
    /// let mut set = [1, 2, 3];
    /// let idxs = [0, 1];
    /// let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
    /// subset.set()[0] = 555;
    /// ```
    /// 
    /// # Errors
    /// OutOfBounds, if any index is `>= set.len()`
    /// 
    /// # Panics
    /// Panics if `std::mem::size_of::<T>() == 0`
    pub fn new(set: &'a mut [T], idxs: &'a [usize]) -> Result<Self, SubsetError> {
        assert_ne!(std::mem::size_of::<T>(), 0);
        let set_size = set.len();
        if idxs.iter().any(|v| *v >= set_size) {
            Err(SubsetError::OutOfBounds)
        } else { Ok(unsafe{Self::new_unchecked(set, idxs)}) }
    }
    /// Constructs a multi-subset from the whole set and indexes of the selected items.
    /// No array bounds check.
    pub unsafe fn new_unchecked(set: &'a mut [T], idxs: &'a [usize]) -> Self {
        Self {
            set: set,
            idxs: idxs
        }
    }
    /// Returns the original slice.
    pub fn set(&mut self) -> &mut [T] {
        self.set
    }
    /// Returns indexes of selected items.
    pub fn idxs(&self) -> &[usize] {
        self.idxs
    }
    /// Checks that no items are selected twice or more.
    /// if `is_unique() == true` then subset can be converted to unique::Subset or unique::SubsetMut.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use subset::multi::*;
    /// 
    /// let mut set = ["one".into(), "two", "three"];
    /// let idxs = [0, 1];
    /// let subset = SubsetMut::new(&mut set, &idxs).unwrap();
    /// if subset.is_unique() {
    ///     let uniq_subset: subset::unique::SubsetMut<_> = subset.try_into().unwrap();
    /// }
    /// ```
    pub fn is_unique(&self) -> bool {
        is_unique(self.idxs)
    }
    /// Converts to `subset::unique::Subset`.
    /// Uniqueness of indexes is not checked.
    pub unsafe fn to_unique_unchecked(self) -> unique::Subset<'a, T> {
        unique::Subset {
            m: self.into()
        }
    }
    /// Converts to `subset::unique::SubsetMut`.
    /// Uniqueness of indexes is not checked.
    pub unsafe fn to_unique_mut_unchecked(self) -> unique::SubsetMut<'a, T> {
        unique::SubsetMut {
            m: self
        }
    }
    /// Returns an iterator over immutable references to selected items.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            ptr: self.set.as_ptr(),
            iter: self.idxs.iter()
        }
    }
}


impl<'a, T> From<unique::SubsetMut<'a, T>> for SubsetMut<'a, T> {
    fn from(s: unique::SubsetMut<T>) -> SubsetMut<T> {
        s.m
    }
}


impl<'a, T> IntoIterator for &'a Subset<'a, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}


impl<'a, T> IntoIterator for &'a SubsetMut<'a, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_set() {
        let set = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let idxs = vec![10];
        assert_eq!(Subset::new(&set, &idxs).err(), Some(SubsetError::OutOfBounds));
        let idxs = vec![2, 2];
        let subset = Subset::new(&set, &idxs).unwrap();
        let result_into: Result<crate::unique::Subset<_>, _> = subset.try_into();
        assert_eq!(result_into.err(), Some(SubsetError::NotUnique));
        let subset = Subset::new(&set, &idxs).unwrap();
        assert_eq!(subset.iter().fold(0, |accum, v| accum + *v), 14);
        let idxs = vec![2, 4, 7];
        let subset = Subset::new(&set, &idxs).unwrap();
        assert!(subset.is_unique());
        let mut sum = 0;
        for e in &subset {
            sum += e;
        }
        assert_eq!(sum, 14);
        let mut sum = 0;
        for e in subset.iter().map(|v| 2*v).rev() {
            sum += e;
        }
        assert_eq!(sum, 28);
        let result_into: Result<crate::unique::Subset<_>, _> = subset.try_into();
        assert!(result_into.is_ok());
    }

    #[test]
    fn test_mut() {
        let mut set = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let idxs = [2, 2];
        let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
        assert!(!subset.is_unique());
        let mut iter = subset.iter();
        let r1 = iter.next().unwrap();
        let r2 = iter.next().unwrap();
        assert!(std::ptr::eq(r1, r2));
        subset.set()[2] = 15;
        let mut iter = subset.iter();
        let r1 = iter.next().unwrap();
        let r2 = iter.next().unwrap();
        assert_eq!(*r1, 15);
        assert_eq!(*r2, 15);
    }
}
