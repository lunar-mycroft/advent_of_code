use std::{collections::HashMap, hash::Hash};

use tap::Pipe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counter<K: Hash + Eq>(HashMap<K, usize>);

impl<K> Counter<K>
where
    K: Hash + Eq,
{
    #[inline]
    pub fn new() -> Self {
        HashMap::default().pipe(Self)
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        HashMap::with_capacity(capacity).pipe(Self)
    }

    #[inline]
    #[must_use]
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, usize> {
        self.0.iter()
    }

    #[inline]
    #[must_use]
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, usize> {
        self.0.keys()
    }

    #[inline]
    #[must_use]
    pub fn into_keys(self) -> std::collections::hash_map::IntoKeys<K, usize> {
        self.0.into_keys()
    }

    #[inline]
    pub fn counts(&self) -> std::iter::Copied<std::collections::hash_map::Values<'_, K, usize>> {
        self.0.values().copied()
    }
}

impl<K> Extend<K> for Counter<K>
where
    K: Hash + Eq,
{
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        for key in iter {
            *self.0.entry(key).or_insert(0) += 1;
        }
    }
}

impl<K> Extend<(K, usize)> for Counter<K>
where
    K: Hash + Eq,
{
    fn extend<T: IntoIterator<Item = (K, usize)>>(&mut self, iter: T) {
        for (key, count) in iter {
            *self.0.entry(key).or_insert(0) += count;
        }
    }
}

impl<K> std::ops::MulAssign<usize> for Counter<K>
where
    K: Hash + Eq,
{
    fn mul_assign(&mut self, rhs: usize) {
        self.0.values_mut().for_each(|v| *v *= rhs);
    }
}

impl<K> std::ops::Mul<usize> for Counter<K>
where
    K: Hash + Eq,
{
    type Output = Self;

    fn mul(mut self, rhs: usize) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<K> std::ops::AddAssign for Counter<K>
where
    K: Hash + Eq,
{
    fn add_assign(&mut self, rhs: Self) {
        for (key, count) in rhs.0 {
            *self.0.entry(key).or_insert(0) += count;
        }
    }
}

impl<K> std::ops::Add for Counter<K>
where
    K: Hash + Eq,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<K> FromIterator<K> for Counter<K>
where
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        iter.into_iter()
            .fold(HashMap::new(), |mut map, s| {
                *map.entry(s).or_insert(0) += 1;
                map
            })
            .pipe(Self)
    }
}

impl<K> IntoIterator for Counter<K>
where
    K: Hash + Eq,
{
    type Item = (K, usize);

    type IntoIter = std::collections::hash_map::IntoIter<K, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, K> IntoIterator for &'a Counter<K>
where
    K: Hash + Eq,
{
    type Item = (&'a K, &'a usize);

    type IntoIter = std::collections::hash_map::Iter<'a, K, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<K> Default for Counter<K>
where
    K: Hash + Eq,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
