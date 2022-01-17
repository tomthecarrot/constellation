use core::marker::PhantomData;
use std::iter::FusedIterator;

pub use generational_arena;
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

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn reserve(&mut self, additional_capacity: usize) {
        self.0.reserve(additional_capacity)
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
pub struct Index<T> {
    inner: ga::Index,
    _phantom: PhantomData<T>,
}
impl<T> Index<T> {
    pub fn new(idx: ga::Index) -> Self {
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

impl<T> Clone for Index<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _phantom: Default::default(),
        }
    }
}

impl<T> Copy for Index<T> {}

impl<T> std::fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Index").field(&self.inner).finish()
    }
}

impl<T> std::hash::Hash for Index<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> std::cmp::PartialEq for Index<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> std::cmp::Eq for Index<T> {}

impl<T> std::cmp::PartialOrd for Index<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T> std::cmp::Ord for Index<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

// ---- Iterator implementations ----

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
        self.0.next().map(|(idx, it)| (Index::new(idx), it))
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

#[cfg(test)]
mod test {
    use super::{ga, Arena, Index};

    #[test]
    fn test_insert_delete() {
        let mut arena = Arena::new();
        assert_eq!(arena.len(), 0);
        assert!(arena.is_empty());
        assert!(arena
            .get(Index::new(ga::Index::from_raw_parts(0, 0)))
            .is_none());
        assert_eq!(arena.iter().count(), 0);
        assert_eq!(arena.iter_mut().count(), 0);

        let a_string = String::from("a");
        let a = arena.insert(a_string.as_str());
        let b = arena.insert("b");
        let c = arena.insert("c");

        assert!(arena.contains(a));
        assert!(arena.contains(b));
        assert!(arena.contains(c));
        assert_eq!(arena[a], "a");
        assert_eq!(arena[b], "b");
        assert_eq!(arena[c], "c");
        assert_eq!(arena.len(), 3);
        assert!(arena.capacity() >= arena.len());
        assert_eq!(arena.iter().count(), 3);
        assert_eq!(arena.iter_mut().count(), 3);

        let b1 = b;

        let removed = arena.remove(b1);
        assert_eq!(removed, Some("b"));
        assert!(arena.contains(a));
        assert!(!arena.contains(b1));
        assert!(arena.contains(c));
        assert_eq!(arena[a], "a");
        assert_eq!(arena.get(b), None);
        assert_eq!(arena.get(c), Some(&"c"));
        assert_eq!(arena.len(), 2);
        assert!(arena.capacity() >= arena.len());
        assert_eq!(arena.iter().count(), 2);
        assert_eq!(arena.iter_mut().count(), 2);

        let b2 = arena.insert("b");

        assert!(arena.contains(a));
        assert!(!arena.contains(b1));
        assert!(arena.contains(b2));
        assert!(arena.contains(c));
        assert_eq!(arena[a], "a");
        assert_eq!(arena.get(b1), None);
        assert_eq!(arena[b2], "b");
        assert_eq!(arena[c], "c");
        assert_eq!(arena.len(), 3);
        assert!(arena.capacity() >= arena.len());
        assert_eq!(arena.iter().count(), 3);
        assert_eq!(arena.iter_mut().count(), 3);
    }
}
