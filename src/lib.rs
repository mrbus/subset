
// TODO: bitvec
use std::collections::HashSet;

/// A very simple subset of vector's items that is able to iterate forward and backward over selected items.
/// 
/// # Examples
///
/// Basic usage:
///
/// ```
/// use subset::*;
/// 
/// let set = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
/// let idxs = vec![2, 2];
/// assert_eq!(Subset::new(&set, &idxs).err(), Some(IndexError::NotUnique));
/// let idxs = vec![10];
/// assert_eq!(Subset::new(&set, &idxs).err(), Some(IndexError::OutOfBounds));
/// let idxs = vec![2, 4, 7];   // Indexes of selected items
/// let subset = Subset::new(&set, &idxs).unwrap();
/// let mut iter = subset.iter();
/// 
/// assert_eq!(Some(&7), iter.next());
/// assert_eq!(Some(&2), iter.next_back());
/// assert_eq!(Some(&5), iter.next_back());
/// assert_eq!(None, iter.next());
/// assert_eq!(None, iter.next_back());
/// ```
pub struct Subset<'a, T> {
    set: &'a Vec<T>,
    idxs: &'a Vec<usize>
}


/// Double-ended iterator over selected items of a vector.
pub struct SubsetIterator<'a, T> {
    subset: &'a Subset<'a, T>,
    iter: std::slice::Iter<'a, usize>
}


#[derive(Debug,PartialEq,Eq)]
pub enum IndexError {
    NotUnique,
    OutOfBounds
}


impl<'a, T> Subset<'a, T> {
    /// Creates a subset from the whole set and indexes of the selected items.
    /// Both the uniqueness of the selected items and the array bounds is checked.
    pub fn new(set: &'a Vec<T>, idxs: &'a Vec<usize>) -> Result<Self, IndexError> {
        let set_size = set.len();
        let uniques: HashSet<usize> = idxs.iter().map(|v| *v).collect();
        if uniques.len() < idxs.len() {
            Err(IndexError::NotUnique)
        } else if idxs.iter().any(|v| *v >= set_size) {
            Err(IndexError::OutOfBounds)
        } else {
            Ok(Self {
                set: set,
                idxs: idxs
            })
        }
    }
    /// Creates a subset from the whole set and indexes of the selected items.
    /// Neither the uniqueness of the selected items, nor the array bounds is checked.
    pub unsafe fn new_unchecked(set: &'a Vec<T>, idxs: &'a Vec<usize>) -> Self {
        Self {
            set: set,
            idxs: idxs
        }
    }
    /// Returns an iterator over subset.
    pub fn iter(&'a self) -> SubsetIterator<'a, T> {
        SubsetIterator {
            subset: self,
            iter: self.idxs.iter()
        }
    }
}


impl<'a, T> Iterator for SubsetIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next().map(|v| &self.subset.set[*v])
    }
}


impl<'a, T> DoubleEndedIterator for SubsetIterator<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back().map(|v| &self.subset.set[*v])
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_set() {
        let set = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let idxs = vec![2, 2];
        assert_eq!(Subset::new(&set, &idxs).err(), Some(IndexError::NotUnique));
        let idxs = vec![10];
        assert_eq!(Subset::new(&set, &idxs).err(), Some(IndexError::OutOfBounds));
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

}
