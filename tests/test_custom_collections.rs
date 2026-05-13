use crepe_support::{CrepeCollections, RelationCollection, RelationMap, RelationSet};

struct VecRelationSet<T>(Vec<T>);

impl<T> Default for VecRelationSet<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T: PartialEq> Extend<T> for VecRelationSet<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for value in iter {
            if !self.0.contains(&value) {
                self.0.push(value);
            }
        }
    }
}

impl<T: PartialEq> RelationSet<T> for VecRelationSet<T> {
    type Iter<'a>
        = std::slice::Iter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn insert(&mut self, value: T) -> bool {
        if self.0.contains(&value) {
            false
        } else {
            self.0.push(value);
            true
        }
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

impl<'a, T> IntoIterator for &'a VecRelationSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

struct VecRelationMap<K, V>(Vec<(K, Vec<V>)>);

impl<K, V> Default for VecRelationMap<K, V> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<K: PartialEq, V> RelationMap<K, V> for VecRelationMap<K, V> {
    type Values = Vec<V>;

    fn get(&self, key: &K) -> Option<&Self::Values> {
        self.0
            .iter()
            .find(|(stored, _)| stored == key)
            .map(|(_, values)| values)
    }

    fn values_mut_or_default(&mut self, key: K) -> &mut Self::Values {
        if let Some(index) = self.0.iter().position(|(stored, _)| stored == &key) {
            &mut self.0[index].1
        } else {
            self.0.push((key, Vec::new()));
            &mut self.0.last_mut().expect("entry was just inserted").1
        }
    }
}

struct VecCollections;

impl CrepeCollections for VecCollections {
    type Set<T> = VecRelationSet<T>;
    type Map<K, V> = VecRelationMap<K, V>;
    type Collection<T> = Vec<T>;
}

#[allow(dead_code)]
fn relation_collection_trait_is_available<T, C: RelationCollection<T>>() {}

mod vector_collections {
    use super::VecCollections;
    use crepe::crepe;

    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    struct Node(i32);

    crepe! {
        @input
        struct Edge<T: Eq>(T, T);

        @output
        struct Tc<T: Eq>(T, T);

        Tc(x, y) <- Edge(x, y);
        Tc(x, z) <- Edge(x, y), Tc(y, z);
    }

    #[test]
    fn custom_collections_use_user_storage() {
        let mut runtime = Crepe::<Node, VecCollections>::new_with_collections();
        runtime.extend([
            Edge(Node(1), Node(2)),
            Edge(Node(2), Node(3)),
            Edge(Node(3), Node(4)),
        ]);

        let (tc,) = runtime.run();
        let mut results: Vec<_> = (&tc).into_iter().map(|tc| (tc.0 .0, tc.1 .0)).collect();
        results.sort_unstable();

        assert_eq!(
            results,
            vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]
        );
    }
}

mod btree_collections {
    use crepe::crepe;

    crepe! {
        @input
        #[derive(Ord, PartialOrd)]
        struct OrderedInput<T: Ord>(T);

        @output
        #[derive(Ord, PartialOrd)]
        struct OrderedOutput<T: Ord>(T);

        OrderedOutput(x) <- OrderedInput(x);
    }

    #[test]
    fn ordered_collections_use_ordered_storage() {
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
        struct OrderedNode(i32);

        let mut runtime =
            Crepe::<OrderedNode, crepe_support::OrderedCrepeCollections>::new_with_collections();
        runtime.extend([OrderedInput(OrderedNode(3)), OrderedInput(OrderedNode(1))]);

        let (output,) = runtime.run();
        let results: Vec<_> = output.iter().map(|value| value.0 .0).collect();

        assert_eq!(results, vec![1, 3]);
    }
}
