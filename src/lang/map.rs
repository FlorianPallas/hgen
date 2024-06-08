use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Clone, PartialEq, Eq)]
pub struct OrderedHashMap<K, V>
where
    K: Hash + Eq + Clone,
{
    inner: HashMap<K, V>,
    order: Vec<K>,
}

impl<K: Debug + Hash + Eq + Clone, V: Debug> Debug for OrderedHashMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K: Hash + Eq + Clone, V> Default for OrderedHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Hash + Eq + Clone, V> OrderedHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.inner.insert(key.clone(), value);
        self.order.push(key);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.order
            .iter()
            .map(move |key| (key, self.inner.get(key).unwrap()))
    }
}

impl<K: Hash + Eq + Clone, V> FromIterator<(K, V)> for OrderedHashMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = Self::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}
