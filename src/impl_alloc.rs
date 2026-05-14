use alloc::vec::Vec;

use crate::RelationCollection;

impl<T> RelationCollection<T> for Vec<T> {
    type Iter<'a>
        = core::slice::Iter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn push(&mut self, value: T) {
        Vec::push(self, value);
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter()
    }
}
