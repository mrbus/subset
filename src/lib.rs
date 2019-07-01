
//! Various subsets of slice's items that are able to iterate forward and backward over references to selected items.

// TODO: use bitvec

// TODO: add compiletest
// Example:
//   let mut set = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
//   let idxs = vec![2, 4, 7];
//   let mut subset = SubsetMut::new(&mut set, &idxs).unwrap();
//   let mut iter = subset.iter_mut();
//   let r2 = iter.next().unwrap();
//   subset.set()[0] = 100;   // MUST NOT COMPILE: Cannot borrow as mutable more than once at a time
//   *r2 = 19;


use std::collections::HashSet;

/// Subset construction or conversion error.
#[derive(Debug,PartialEq,Eq)]
pub enum SubsetError {
    NotUnique,
    OutOfBounds
}

fn is_unique(array: &[usize]) -> bool {
    let mut uniques: HashSet<usize> = HashSet::with_capacity(array.len());
    array.iter().all(|idx| uniques.insert(*idx))
}


pub mod unique;
pub mod multi;
