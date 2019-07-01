
//! Various subsets of slice's items that are able to iterate forward and backward over references to selected items.

// TODO: use bitvec
// TODO: add compiletest

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


// TODO тесты
