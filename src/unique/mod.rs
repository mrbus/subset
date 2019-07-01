//! Subset of slice's items that is able to iterate forward and backward over mutable or immutable references to selected items.
//! Each item of a slice can be selected no more than once.
//! 
//! # Examples
//!
//! ```
//! use subset::unique::*;
//! 
//! // Constructing mutable subset
//! let mut set = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
//! let idxs = vec![10];
//! assert_eq!(SubsetMut::new(&mut set, &idxs).err(), Some(SubsetError::OutOfBounds));
//! let idxs = vec![2, 2, 5];
//! assert_eq!(SubsetMut::new(&mut set, &idxs).err(), Some(SubsetError::NotUnique));
//! let idxs = vec![2, 4, 7];   // Indexes of selected items
//! let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
//! 
//! // Iterating over mutable references
//! let mut iter = subset.iter_mut();
//! let r1 = iter.next().unwrap();
//! assert_eq!(*r1, 7);
//! let r2 = iter.next().unwrap();
//! assert_eq!(*r2, 5);
//! *r1 = 19;
//! *r2 = 33;
//! assert_eq!(iter.next(), Some(&mut 2));
//! assert_eq!(iter.next(), None);
//! assert_eq!(subset.set(), vec![9, 8, 19, 6, 33, 4, 3, 2, 1, 0].as_slice());
//! 
//! // Converting to immutable subset
//! let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
//! let imm_subset: Subset<_> = subset.into();
//! 
//! // Converting to mutable or immutable multi-subset
//! let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
//! let multi_subset: subset::multi::SubsetMut<_> = subset.into();
//! let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
//! let imm_multi_subset: subset::multi::Subset<_> = subset.into();
//! ```

pub use std::convert::{From,Into,TryFrom,TryInto};
use super::{is_unique, multi};
pub use super::SubsetError;

/// Subset of slice's items that is able to iterate forward and backward over immutable references to selected items.
/// Each item of a slice can be selected no more than once.
// Just a wrapper over multi::Subset
#[derive(Debug)]
pub struct Subset<'a, T> {
    pub(crate) m: multi::Subset<'a, T>
}


impl<'a, T> Subset<'a, T> {
    /// Constructs a subset from the whole set and indexes of the selected items.
    /// Both the uniqueness of the selected items and the array bounds is checked.
    /// Note that subsets are not designed for ZSTs.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use subset::unique::*;
    /// let set = [1.0, 1.1, 1.2];
    /// let idxs = [0, 1];
    /// let subset = Subset::new(&set, &idxs).unwrap();
    /// ```
    /// 
    /// # Errors
    /// NotUnique, if there are duplicate indexes.
    /// OutOfBounds, if any index is `>= set.len()`.
    /// 
    /// # Panics
    /// Panics if `std::mem::size_of::<T>() == 0`
    pub fn new(set: &'a [T], idxs: &'a [usize]) -> Result<Self, SubsetError> {
        // TODO переделать под into()
        multi::Subset::new(set, idxs)?.try_into()
    }
    /// Constructs a subset from the whole set and indexes of the selected items.
    /// Neither the uniqueness of the selected items, nor the array bounds is checked.
    pub unsafe fn new_unchecked(set: &'a [T], idxs: &'a [usize]) -> Self {
        multi::Subset::new_unchecked(set, idxs).to_unique_unchecked()
    }
    /// Returns the original slice.
    pub fn set(&self) -> &[T] {
        self.m.set()
    }
    /// Returns indexes of selected items.
    pub fn idxs(&self) -> &[usize] {
        self.m.idxs()
    }
    /// Returns an iterator over immutable references to selected items.
    pub fn iter(&self) -> multi::Iter<T> {
        self.m.iter()
    }
}


impl<'a, T> From<SubsetMut<'a, T>> for Subset<'a, T> {
    fn from(s: SubsetMut<'a, T>) -> Self {
        Self {
            m: s.m.into()
        }
    }
}


impl<'a, T> TryFrom<multi::Subset<'a, T>> for Subset<'a, T> {
    type Error = SubsetError;
    fn try_from(s: multi::Subset<'a, T>) -> Result<Self, SubsetError> {
        if is_unique(s.idxs) {
            Ok(unsafe{s.to_unique_unchecked()})
        } else {
            Err(SubsetError::NotUnique)
        }
    }
}


impl<'a, T> TryFrom<multi::SubsetMut<'a, T>> for Subset<'a, T> {
    type Error = SubsetError;
    fn try_from(s: multi::SubsetMut<'a, T>) -> Result<Self, SubsetError> {
        if is_unique(s.idxs) {
            Ok(unsafe{s.to_unique_unchecked()})
        } else {
            Err(SubsetError::NotUnique)
        }
    }
}


impl<'a, T> IntoIterator for &'a Subset<'a, T> {
    type Item = &'a T;
    type IntoIter = multi::Iter<'a, T>;
    fn into_iter(self) -> multi::Iter<'a, T> {
        self.iter()
    }
}


/// Subset of slice's items that is able to iterate forward and backward over mutable or immutable references to selected items.
/// Each item of a slice can be selected no more than once.
// Just a wrapper over multi::SubsetMut
#[derive(Debug)]
pub struct SubsetMut<'a, T> {
    pub(crate) m: multi::SubsetMut<'a, T>
}

/// Double-ended iterator over mutable references to selected items of set.
pub struct IterMut<'a, T> {
    ptr: *mut T,    // Points to the set
    iter: std::slice::Iter<'a, usize>
}

impl<'a, T> SubsetMut<'a, T> {
    /// Constructs a subset from the whole set and indexes of the selected items.
    /// Both the uniqueness of the selected items and the array bounds is checked.
    /// Note that subsets are not designed for ZSTs.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use subset::unique::*;
    /// let mut set = [1.0, 1.1, 1.2];
    /// let idxs = [0, 1];
    /// let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
    /// subset.set()[0] = 3.1415;
    /// ```
    /// 
    /// # Errors
    /// NotUnique, if there are duplicate indexes.
    /// OutOfBounds, if any index is `>= set.len()`.
    /// 
    /// # Panics
    /// Panics if `std::mem::size_of::<T>() == 0`
    pub fn new(set: &'a mut [T], idxs: &'a [usize]) -> Result<Self, SubsetError> {
        multi::SubsetMut::new(set, idxs)?.try_into()
    }
    /// Constructs a subset from the whole set and indexes of the selected items.
    /// Neither the uniqueness of the selected items, nor the array bounds is checked.
    pub unsafe fn new_unchecked(set: &'a mut [T], idxs: &'a [usize]) -> Self {
        multi::SubsetMut::new_unchecked(set, idxs).to_unique_mut_unchecked()
    }
    /// Returns the original slice.
    pub fn set(&mut self) -> &mut [T] {
        self.m.set()
    }
    /// Returns indexes of selected items.
    pub fn idxs(&self) -> &[usize] {
        self.m.idxs()
    }
    /// Returns an iterator over immutable references to selected items.
    pub fn iter(&self) -> multi::Iter<T> {
        self.m.iter()
    }
    /// Returns an iterator over mutable references to selected items.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            ptr: self.m.set.as_mut_ptr(),
            iter: self.m.idxs.iter()
        }
    }
}


impl<'a, T> TryFrom<multi::SubsetMut<'a, T>> for SubsetMut<'a, T> {
    type Error = SubsetError;
    fn try_from(s: multi::SubsetMut<'a, T>) -> Result<Self, SubsetError> {
        if is_unique(s.idxs) {
            Ok(unsafe{s.to_unique_mut_unchecked()})
        } else {
            Err(SubsetError::NotUnique)
        }
    }
}


impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        unsafe {
            match self.iter.next() {
                None => None,
                Some(idx) => Some(&mut *self.ptr.offset(*idx as isize))
            }
        }
    }
}


impl<'a, T: 'a> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut T> {
        unsafe {
            match self.iter.next_back() {
                None => None,
                Some(idx) => Some(&mut *self.ptr.offset(*idx as isize))
            }
        }
    }
}


impl<'a, T> IntoIterator for &'a SubsetMut<'a, T> {
    type Item = &'a T;
    type IntoIter = multi::Iter<'a, T>;
    fn into_iter(self) -> multi::Iter<'a, T> {
        self.iter()
    }
}


impl<'a, T> IntoIterator for &'a mut SubsetMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_set() {
        let set = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let idxs = vec![2, 2];
        assert_eq!(Subset::new(&set, &idxs).err(), Some(SubsetError::NotUnique));
        let idxs = vec![10];
        assert_eq!(Subset::new(&set, &idxs).err(), Some(SubsetError::OutOfBounds));
        let idxs = vec![2, 4, 7];
        let subset = Subset::new(&set, &idxs).unwrap();
        let mut sum = 0;
        for e in subset.iter() {
            sum += e;
        }
        assert_eq!(sum, 14);
        let mut sum = 0;
        for e in subset.iter().map(|v| 2*v).rev() {
            sum += e;
        }
        assert_eq!(sum, 28);
    }

    #[test]
    fn test_mut() {
        let mut set = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let idxs = vec![2, 2];
        assert_eq!(SubsetMut::new(&mut set, &idxs).err(), Some(SubsetError::NotUnique));
        let idxs = vec![10];
        assert_eq!(SubsetMut::new(&mut set, &idxs).err(), Some(SubsetError::OutOfBounds));
        let idxs = vec![2, 4, 7];
        let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
        let mut iter = subset.iter_mut();
        let r1 = iter.next().unwrap();
        let r2 = iter.next().unwrap();
        *r1 = 19;
        *r2 = 33;
        assert_eq!(subset.set(), vec![9, 8, 19, 6, 33, 4, 3, 2, 1, 0].as_slice());
        let mut sum = 0;
        for e in subset.iter().map(|v| 2* *v).rev() {
            sum += e;
        }
        assert_eq!(sum, 108);
    }
}
