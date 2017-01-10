//! Freelist data structure
//!
//! A Freelist is a combination of a `VecMap` and `Vec`. Elements are stored in the `VecMap`,
//! and empty elements are stored in the `Vec`.
//!
//! This structure is useful because the indexes returned by `.add` are not invalidated when other elements are added or removed,
//! because nothing is reordered.
//!
//! When an element is removed, its index is placed in the `Vec`, the "freelist". Then when another element is added,
//! the freelist is checked and it just pops off an index and inserts the new element in the spot of an old entry.
use std::ops::{Index, IndexMut};
use vec_map::{VecMap, Keys, Iter, IterMut};

/// Freelist data structure as described above.
///
/// Elements can be added by called `.add`, which then returns the index in which it can be accessed from.
#[derive(PartialEq, Eq, Clone)]
pub struct Freelist<T> {
    freelist: Vec<usize>,
    map: VecMap<T>,
}

impl<T> Default for Freelist<T> {
    #[inline(always)]
    fn default() -> Freelist<T> {
        Freelist::new()
    }
}

impl<T> Index<usize> for Freelist<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index).expect("no entry found for key")
    }
}

impl<T> IndexMut<usize> for Freelist<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("no entry found for key")
    }
}

impl<T> Freelist<T> {
    /// Create a new empty `Freelist`
    pub fn new() -> Freelist<T> {
        Freelist {
            freelist: Vec::new(),
            map: VecMap::new()
        }
    }

    /// Create a new `Freelist` with pre-allocated capacity
    pub fn with_capacity(cap: usize) -> Freelist<T> {
        Freelist {
            freelist: Vec::with_capacity(cap),
            map: VecMap::with_capacity(cap)
        }
    }

    /// Add a new element to the `Freelist` and return its index
    pub fn add(&mut self, value: T) -> usize {
        let index = if let Some(index) = self.freelist.pop() { index } else {
            self.map.len()
        };

        self.map.insert(index, value);

        index
    }

    /// Remove an element at the given index and return the value
    ///
    /// `None` is returned if the the index contained no element.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let res = self.map.remove(index);

        if res.is_some() || !self.freelist.contains(&index) {
            self.freelist.push(index);
        }

        res
    }

    /// Returns true if there are no elements in the `Freelist`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the number of elements in the `Freelist`
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Get a reference to an element at the given index.
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.map.get(index)
    }

    /// Get a mutable reference to an element at the given index.
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.map.get_mut(index)
    }

    /// Clear the `Freelist` of all elements.
    pub fn clear(&mut self) {
        self.freelist.clear();
        self.map.clear();
    }

    /// Return a `Keys` iterator for the `Freelist`
    #[inline(always)]
    pub fn keys(&self) -> Keys<T> {
        self.map.keys()
    }

    /// Return an iterator over the values in the `Freelist`
    #[inline(always)]
    pub fn iter(&self) -> Iter<T> {
        self.map.iter()
    }

    /// Return a mutable iterator over the values in the `Freelist`
    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.map.iter_mut()
    }
}