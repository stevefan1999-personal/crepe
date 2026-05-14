use alloc::vec::Vec;
use core::hash::{BuildHasher, Hash};
use std::collections::{HashMap, HashSet};

use crate::{CrepeCollections, RelationMap, RelationSet};

impl<T, S> RelationSet<T> for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher + Default,
{
    type Iter<'a>
        = std::collections::hash_set::Iter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn insert(&mut self, value: T) -> bool {
        HashSet::insert(self, value)
    }

    fn contains(&self, value: &T) -> bool {
        HashSet::contains(self, value)
    }

    fn is_empty(&self) -> bool {
        HashSet::is_empty(self)
    }

    fn iter(&self) -> Self::Iter<'_> {
        HashSet::iter(self)
    }
}

impl<K, V, S> RelationMap<K, V> for HashMap<K, Vec<V>, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    type Values = Vec<V>;

    fn get(&self, key: &K) -> Option<&Self::Values> {
        HashMap::get(self, key)
    }

    fn values_mut_or_default(&mut self, key: K) -> &mut Self::Values {
        self.entry(key).or_default()
    }
}

/// Crepe collection family backed by standard-library hash maps and sets.
pub struct StdCrepeCollections;

impl CrepeCollections for StdCrepeCollections {
    type Set<T> = HashSet<T>;
    type Map<K, V> = HashMap<K, Vec<V>>;
    type Collection<T> = Vec<T>;
}
