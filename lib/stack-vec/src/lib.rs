#![no_std]

use core::ops::{Deref, DerefMut};
use core::iter::{IntoIterator};

/// A stack-based vector with a fixed capacity.
#[derive(Debug)]
pub struct StackVec<'a, T: 'a> {
    storage: &'a mut [T],
    len: usize,
}

impl<'a, T: 'a> StackVec<'a, T> {
    /// Constructs a new, empty `StackVec<T>` using `storage` as the backing store.
    pub fn new(storage: &'a mut [T]) -> StackVec<'a, T> {
        StackVec { storage, len: 0 }
    }

    /// Constructs a new `StackVec<T>` using `storage` with `len` pre-filled.
    /// # Panics
    /// Panics if `len > storage.len()`.
    pub fn with_len(storage: &'a mut [T], len: usize) -> StackVec<'a, T> {
        assert!(len <= storage.len(), "Length exceeds storage capacity");
        StackVec { storage, len }
    }

    /// Returns the number of elements this vector can hold.
    pub fn capacity(&self) -> usize {
        self.storage.len()
    }

    /// Shortens the vector, keeping the first `len` elements.
    pub fn truncate(&mut self, len: usize) {
        if len < self.len {
            self.len = len;
        }
    }

    /// Extracts a slice containing the entire vector, consuming `self`.
    pub fn into_slice(self) -> &'a mut [T] {
        &mut self.storage[..self.len]
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        &self.storage[..self.len]
    }

    /// Extracts a mutable slice of the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.storage[..self.len]
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns true if the vector is at capacity.
    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    /// Appends `value` to the back of this vector if the vector is not full.
    ///
    /// # Error
    /// If this vector is full, an `Err(())` is returned.
    pub fn push(&mut self, value: T) -> Result<(), ()> {
        if self.is_full() {
            return Err(());
        }

        self.storage[self.len] = value;
        self.len += 1;
        Ok(())
    }
}

impl<'a, T: Clone + 'a> StackVec<'a, T> {
    /// Removes the last element from this vector by cloning it and returns it.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.len -= 1;
        Some(self.storage[self.len].clone())
    }
}

impl<'a, T: 'a> Deref for StackVec<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T: 'a> DerefMut for StackVec<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

// Implement `IntoIterator` for owned `StackVec`
impl<'a, T: 'a> IntoIterator for StackVec<'a, T> {
    type Item = T;
    type IntoIter = StackVecIntoIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackVecIntoIter {
            vec: self,
            index: 0,
        }
    }
}

/// An iterator that consumes the `StackVec`.
pub struct StackVecIntoIter<'a, T: 'a> {
    vec: StackVec<'a, T>,
    index: usize,
}

impl<'a, T: 'a> Iterator for StackVecIntoIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let val = core::mem::replace(&mut self.vec.storage[self.index], unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            });
            self.index += 1;
            Some(val)
        } else {
            None
        }
    }
}

// Implement `IntoIterator` for borrowed `&StackVec`
impl<'a, T: 'a> IntoIterator for &'a StackVec<'a, T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

// Implement `IntoIterator` for mutable `&mut StackVec`
impl<'a, T: 'a> IntoIterator for &'a mut StackVec<'a, T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}
