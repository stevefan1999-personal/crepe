use core::hash::{BuildHasher, Hash};

use crate::{CrepeCollections, RelationCollection, RelationMap, RelationSet};

impl<T, const N: usize> RelationCollection<T> for heapless::Vec<T, N> {
    type Iter<'a>
        = core::slice::Iter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn push(&mut self, value: T) {
        heapless::Vec::push(self, value)
            .unwrap_or_else(|_| panic!("crepe heapless relation collection capacity exceeded"));
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter()
    }
}

/// Relation set backed by a fixed-capacity heapless index set.
#[derive(Clone, Debug)]
pub struct HeaplessRelationSet<T, const N: usize>(heapless::index_set::FnvIndexSet<T, N>);

impl<T, const N: usize> Default for HeaplessRelationSet<T, N> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T, const N: usize> HeaplessRelationSet<T, N>
where
    T: Eq + Hash,
{
    /// Return the number of stored relation values.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return whether this relation set contains no values.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return whether the relation value is already present.
    pub fn contains(&self, value: &T) -> bool {
        self.0.contains(value)
    }

    /// Iterate over stored relation values.
    pub fn iter(&self) -> heapless::index_set::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T, const N: usize> RelationSet<T> for HeaplessRelationSet<T, N>
where
    T: Eq + Hash,
{
    type Iter<'a>
        = heapless::index_set::Iter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn insert(&mut self, value: T) -> bool {
        self.0
            .insert(value)
            .unwrap_or_else(|_| panic!("crepe heapless relation set capacity exceeded"))
    }

    fn contains(&self, value: &T) -> bool {
        self.0.contains(value)
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter()
    }
}

impl<T, const N: usize> Extend<T> for HeaplessRelationSet<T, N>
where
    T: Eq + Hash,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for value in iter {
            RelationSet::insert(self, value);
        }
    }
}

impl<T, const N: usize> IntoIterator for HeaplessRelationSet<T, N>
where
    T: Eq + Hash + Copy,
{
    type Item = T;
    type IntoIter = <heapless::Vec<T, N> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut values = heapless::Vec::new();
        for value in self.0.iter().copied() {
            values
                .push(value)
                .unwrap_or_else(|_| panic!("crepe heapless relation set capacity exceeded"));
        }
        values.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a HeaplessRelationSet<T, N>
where
    T: Eq + Hash,
{
    type Item = &'a T;
    type IntoIter = heapless::index_set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T, S, const N: usize> RelationSet<T> for heapless::index_set::IndexSet<T, S, N>
where
    T: Eq + Hash,
    S: BuildHasher + Default,
{
    type Iter<'a>
        = heapless::index_set::Iter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn insert(&mut self, value: T) -> bool {
        heapless::index_set::IndexSet::insert(self, value)
            .unwrap_or_else(|_| panic!("crepe heapless relation set capacity exceeded"))
    }

    fn contains(&self, value: &T) -> bool {
        heapless::index_set::IndexSet::contains(self, value)
    }

    fn is_empty(&self) -> bool {
        heapless::index_set::IndexSet::is_empty(self)
    }

    fn iter(&self) -> Self::Iter<'_> {
        heapless::index_set::IndexSet::iter(self)
    }
}

impl<K, V, S, const N: usize, const M: usize> RelationMap<K, V>
    for heapless::index_map::IndexMap<K, heapless::Vec<V, M>, S, N>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    type Values = heapless::Vec<V, M>;

    fn get(&self, key: &K) -> Option<&Self::Values> {
        heapless::index_map::IndexMap::get(self, key)
    }

    fn values_mut_or_default(&mut self, key: K) -> &mut Self::Values {
        self.entry(key)
            .or_default()
            .unwrap_or_else(|_| panic!("crepe heapless relation map capacity exceeded"))
    }
}

/// Crepe collection family backed by fixed-capacity heapless collections.
pub struct HeaplessCrepeCollections<const N: usize>;

impl<const N: usize> CrepeCollections for HeaplessCrepeCollections<N> {
    type Set<T> = HeaplessRelationSet<T, N>;
    type Map<K, V> = heapless::index_map::FnvIndexMap<K, heapless::Vec<V, N>, N>;
    type Collection<T> = heapless::Vec<T, N>;
}
