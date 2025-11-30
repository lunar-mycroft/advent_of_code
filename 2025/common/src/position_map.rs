use std::collections::BTreeMap;

use glam::IVec2;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PositionMap<T>(BTreeMap<i32, BTreeMap<i32, T>>);

impl<T> PositionMap<T> {
    #[must_use]
    pub fn get(&self, pos: IVec2) -> Option<&T> {
        self.0.get(&pos.y)?.get(&pos.x)
    }

    pub fn insert(&mut self, pos: IVec2, value: T) -> Option<T> {
        self.0.entry(pos.y).or_default().insert(pos.x, value)
    }

    pub fn remove(&mut self, pos: &IVec2) -> Option<T> {
        let res = self.0.get_mut(&pos.y)?.remove(&pos.x);
        if self.0.get(&pos.y)?.is_empty() {
            self.0.remove(&pos.y);
        }
        res
    }
}
