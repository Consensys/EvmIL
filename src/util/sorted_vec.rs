use std::{cmp,fmt,ops};

pub type SortedVecIter<'a,T> = std::slice::Iter<'a,T>;
pub type SortedVecIterMut<'a,T> = std::slice::IterMut<'a,T>;

/// A vector where all elements are maintained in sorted order
/// (without duplicates).  This allows for efficient lookup and
/// duplicate removal.
#[derive(Clone,PartialEq,Eq,Ord,PartialOrd)]
pub struct SortedVec<T:Ord> {
    items: Vec<T>
}

impl<T:Ord+Clone> SortedVec<T> {
    pub const fn new() -> Self { SortedVec{items: Vec::new()} }

    /// Get the number of items in this sorted vector
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Get item at given index in vector
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Get mutable item at given index in vector
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    pub fn iter<'a>(&'a self) -> SortedVecIter<'a,T> {
        self.items.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> SortedVecIterMut<'a,T> {
        self.items.iter_mut()
    }

    /// Check whether a given item is already contained in the sorted
    /// vector.
    pub fn contains(&self, item: T) -> bool {
        match self.items.binary_search(&item) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    /// Insert a new item into this sorted vector, and indicate
    /// whether or not it was actually added.  The item will not be
    /// added if a duplicate already exists.
    pub fn insert(&mut self, item: T) -> bool {
        // Find position where item should be inserted.
        match self.items.binary_search(&item) {
            Ok(_) => false,
            Err(i) => {
                self.items.insert(i,item);
                true
            }
        }
    }

    /// Insert zero or more elements into this sorted vector.
    pub fn insert_all(&mut self, other: &SortedVec<T>) -> bool {
        let mut changed = false;
        // FIXME: performance could be improved!
        for v in &other.items {
            changed |= self.insert(v.clone());
        }
        // done
        changed
    }
}

impl<T:Ord+fmt::Debug> fmt::Debug for SortedVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

impl<T:Ord+Clone> From<&[T]> for SortedVec<T> {
    fn from(items: &[T]) -> Self {
        // Convert slide into vec
        let mut vec = items.to_vec();
        // Sort vec
        vec.sort_unstable();
        // Deduplicate
        vec.dedup();
        // Done
        SortedVec{items: vec}
    }
}

impl<T:Ord+Clone> From<Vec<T>> for SortedVec<T> {
    fn from(mut items: Vec<T>) -> Self {
        // Sort vec
        items.sort_unstable();
        // Deduplicate
        items.dedup();
        // Done
        SortedVec{items}
    }
}

impl<T:Ord+Clone> cmp::PartialEq<[T]> for SortedVec<T> {
    fn eq(&self, other: &[T]) -> bool {
        &self.items == other
    }
}

impl<T:Ord+Clone> cmp::PartialEq<Vec<T>> for SortedVec<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        &self.items == other
    }
}

impl<T:Ord+Clone> ops::Index<usize> for SortedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T:Ord+Clone> ops::IndexMut<usize> for SortedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<'a,T:Ord> IntoIterator for &'a SortedVec<T> {
    type Item = &'a T;
    type IntoIter = SortedVecIter<'a,T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!("got here");
    }
}
