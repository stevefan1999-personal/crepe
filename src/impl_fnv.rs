use alloc::vec::Vec;

use crate::CrepeCollections;

#[cfg(feature = "std")]
type FnvHashSet<T> = std::collections::HashSet<T, fnv::FnvBuildHasher>;

#[cfg(feature = "std")]
type FnvHashMap<K, V> = std::collections::HashMap<K, V, fnv::FnvBuildHasher>;

#[cfg(not(feature = "std"))]
type FnvHashSet<T> = hashbrown::HashSet<T, fnv::FnvBuildHasher>;

#[cfg(not(feature = "std"))]
type FnvHashMap<K, V> = hashbrown::HashMap<K, V, fnv::FnvBuildHasher>;

/// Crepe collection family backed by FNV hash maps and sets.
pub struct FnvCrepeCollections;

impl CrepeCollections for FnvCrepeCollections {
    type Set<T> = FnvHashSet<T>;
    type Map<K, V> = FnvHashMap<K, Vec<V>>;
    type Collection<T> = Vec<T>;
}
