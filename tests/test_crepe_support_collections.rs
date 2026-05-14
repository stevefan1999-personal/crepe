#[cfg(feature = "alloc")]
#[test]
fn vec_is_only_a_relation_collection() {
    use crepe::RelationCollection;

    let mut values = Vec::new();
    values.push(10);
    RelationCollection::push(&mut values, 11);

    assert_eq!(values.iter().copied().collect::<Vec<_>>(), vec![10, 11]);
}

#[cfg(feature = "hashbrown")]
#[test]
fn generated_runtime_accepts_hashbrown_collections() {
    use crepe::crepe;

    crepe! {
        @input
        struct Edge(i32, i32);

        @output
        struct Reachable(i32, i32);

        Reachable(x, y) <- Edge(x, y);
        Reachable(x, z) <- Edge(x, y), Reachable(y, z);
    }

    let mut runtime = Crepe::<crepe::HashbrownCrepeCollections>::new_with_collections();
    runtime.extend([Edge(1, 2), Edge(2, 3)]);

    let (reachable,) = runtime.run();
    assert!(reachable.contains(&Reachable(1, 3)));
}

#[cfg(feature = "std")]
#[test]
fn std_hash_collections_satisfy_relation_traits() {
    use crepe::RelationMap as _;
    use std::collections::{HashMap, HashSet};

    let mut set: HashSet<u32> = Default::default();
    assert!(set.insert(1));
    assert!(!set.insert(1));
    assert!(set.contains(&1));

    let mut map: HashMap<(u32,), Vec<u32>> = Default::default();
    map.push((1,), 10);
    map.push((1,), 11);
    assert_eq!(
        map.iter_key(&(1,))
            .expect("key should exist")
            .copied()
            .collect::<Vec<_>>(),
        vec![10, 11]
    );
}

#[cfg(feature = "std")]
#[test]
fn generated_runtime_accepts_std_hash_collections() {
    use crepe::{crepe, CrepeCollections};
    use std::collections::{HashMap, HashSet};

    struct HashCollections;

    impl CrepeCollections for HashCollections {
        type Set<T> = HashSet<T>;
        type Map<K, V> = HashMap<K, Vec<V>>;
        type Collection<T> = Vec<T>;
    }

    crepe! {
        @input
        struct Edge(i32, i32);

        @output
        struct Reachable(i32, i32);

        Reachable(x, y) <- Edge(x, y);
        Reachable(x, z) <- Edge(x, y), Reachable(y, z);
    }

    let mut runtime = Crepe::<HashCollections>::new_with_collections();
    runtime.extend([Edge(1, 2), Edge(2, 3)]);

    let (reachable,) = runtime.run();
    assert!(reachable.contains(&Reachable(1, 3)));
}

#[cfg(feature = "heapless")]
#[test]
fn generated_runtime_accepts_heapless_collections() {
    use crepe::crepe;

    crepe! {
        @input
        struct Edge(i32, i32);

        @output
        struct Reachable(i32, i32);

        Reachable(x, y) <- Edge(x, y);
        Reachable(x, z) <- Edge(x, y), Reachable(y, z);
    }

    let mut runtime = Crepe::<crepe::HeaplessCrepeCollections<16>>::new_with_collections();
    runtime.extend([Edge(1, 2), Edge(2, 3)]);

    let (reachable,) = runtime.run();
    assert!(reachable.contains(&Reachable(1, 3)));
}
