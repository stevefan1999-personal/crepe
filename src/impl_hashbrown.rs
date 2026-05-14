use alloc::vec::Vec;
use core::hash::{BuildHasher, Hash};

use crate::{CrepeCollections, RelationMap, RelationSet};

impl<T, S> RelationSet<T> for hashbrown::HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher + Default,
{
    type Iter<'a>
        = hashbrown::hash_set::Iter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn insert(&mut self, value: T) -> bool {
        hashbrown::HashSet::insert(self, value)
    }

    fn contains(&self, value: &T) -> bool {
        hashbrown::HashSet::contains(self, value)
    }

    fn is_empty(&self) -> bool {
        hashbrown::HashSet::is_empty(self)
    }

    fn iter(&self) -> Self::Iter<'_> {
        hashbrown::HashSet::iter(self)
    }
}

impl<K, V, S> RelationMap<K, V> for hashbrown::HashMap<K, Vec<V>, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    type Values = Vec<V>;

    fn get(&self, key: &K) -> Option<&Self::Values> {
        hashbrown::HashMap::get(self, key)
    }

    fn values_mut_or_default(&mut self, key: K) -> &mut Self::Values {
        self.entry(key).or_default()
    }
}

/// Crepe collection family backed by hashbrown maps and sets with FNV hashing.
pub struct HashbrownCrepeCollections;

impl CrepeCollections for HashbrownCrepeCollections {
    type Set<T> = hashbrown::HashSet<T, fnv::FnvBuildHasher>;
    type Map<K, V> = hashbrown::HashMap<K, Vec<V>, fnv::FnvBuildHasher>;
    type Collection<T> = Vec<T>;
}
