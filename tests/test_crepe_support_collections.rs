use crepe_support::{
    OrderedCrepeCollections, RelationMap, RelationMapOf, RelationSet, RelationSetOf, VecRelationSet,
};

#[test]
fn vec_relation_set_helpers_cover_duplicate_and_borrowed_paths() {
    let mut existing = VecRelationSet::default();
    assert!(existing.is_empty());
    assert!(existing.insert(1));
    assert!(!existing.is_empty());
    assert!(existing.contains(&1));
    assert_eq!(existing.iter().copied().collect::<Vec<_>>(), vec![1]);
    assert_eq!(
        (&existing).into_iter().copied().collect::<Vec<_>>(),
        vec![1]
    );

    let mut update = VecRelationSet::default();
    assert!(!update.insert_if_missing(&existing, 1));
    assert!(update.insert_if_missing(&existing, 2));
    assert_eq!(update.into_iter().collect::<Vec<_>>(), vec![2]);

    let mut extended = VecRelationSet::default();
    Extend::extend(&mut extended, [3, 3, 4]);
    assert_eq!(extended.into_iter().collect::<Vec<_>>(), vec![3, 4]);
}

#[test]
fn ordered_relation_map_uses_btree_storage() {
    let mut map: RelationMapOf<OrderedCrepeCollections, (u32,), u32> = Default::default();
    assert!(map.is_key_absent(&(1,)));

    map.push((1,), 10);
    map.push((1,), 11);
    assert_eq!(
        map.iter_key(&(1,))
            .expect("key should exist")
            .copied()
            .collect::<Vec<_>>(),
        vec![10, 11]
    );

    let mut set: RelationSetOf<OrderedCrepeCollections, u32> = Default::default();
    set.insert(20);
    set.insert(21);
    map.extend_from_set(&set, |value| (value / 10,));
    assert_eq!(
        map.iter_key(&(2,))
            .expect("key should exist")
            .copied()
            .collect::<Vec<_>>(),
        vec![20, 21]
    );
}
