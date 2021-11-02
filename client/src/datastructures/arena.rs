use core::marker::PhantomData;
use std::iter::FusedIterator;

use generational_arena as ga;

/// A generational arena allocator. Built as a wrapper around
/// [`generational_arena::Arena`], but with a type-safe index.
#[derive(Debug, Clone)]
pub struct Arena<T>(ga::Arena<T>);
impl<T> Arena<T> {
    pub fn new() -> Self {
        Self(ga::Arena::new())
    }

    pub fn insert(&mut self, value: T) -> Index<T> {
        Index::new(self.0.insert(value))
    }

    pub fn get(&self, idx: Index<T>) -> Option<&T> {
        self.0.get(idx.into())
    }

    pub fn get_mut(&mut self, idx: Index<T>) -> Option<&mut T> {
        self.0.get_mut(idx.into())
    }

    pub fn with_capacity(n: usize) -> Self {
        Self(ga::Arena::with_capacity(n))
    }

    pub fn remove(&mut self, idx: Index<T>) -> Option<T> {
        self.0.remove(idx.into())
    }

    pub fn contains(&mut self, idx: Index<T>) -> bool {
        self.0.contains(idx.into())
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self.0.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(self.0.iter_mut())
    }
}
impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<T> IntoIterator for Arena<T> {
    type Item = T;

    type IntoIter = ga::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<T> From<Arena<T>> for ga::Arena<T> {
    fn from(arena: Arena<T>) -> Self {
        arena.0
    }
}
impl<T> core::ops::Index<Index<T>> for Arena<T> {
    type Output = T;

    fn index(&self, index: Index<T>) -> &Self::Output {
        &self.0[index.into()]
    }
}
impl<T> core::ops::IndexMut<Index<T>> for Arena<T> {
    fn index_mut(&mut self, index: Index<T>) -> &mut Self::Output {
        &mut self.0[index.into()]
    }
}
impl<T> FromIterator<T> for Arena<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let arena = ga::Arena::from_iter(iter);
        Self(arena)
    }
}

/// A type-safe index for [`Arena`]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Index<T> {
    inner: ga::Index,
    _phantom: PhantomData<T>,
}
impl<T> Index<T> {
    fn new(idx: ga::Index) -> Self {
        Self {
            inner: idx,
            _phantom: PhantomData,
        }
    }
}
impl<T> From<Index<T>> for generational_arena::Index {
    fn from(idx: Index<T>) -> Self {
        idx.inner
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a, T: 'a>(ga::Iter<'a, T>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Index<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(idx, it)| (Index::new(idx), it))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|(idx, it)| (Index::new(idx), it))
    }
}
impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'a, T> FusedIterator for Iter<'a, T> {}
impl<'a, T> IntoIterator for &'a mut Arena<T> {
    type Item = (Index<T>, &'a mut T);

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T: 'a>(ga::IterMut<'a, T>);
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (Index<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|(idx, it)| (Index::new(idx), it))
    }
}
impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'a, T> FusedIterator for IterMut<'a, T> {}
