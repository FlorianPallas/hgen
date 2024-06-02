use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderedHashMap<K, V>
where
    K: Hash + Eq + Clone,
{
    inner: HashMap<K, V>,
    order: Vec<K>,
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

    pub fn extend(&mut self, other: Self) {
        self.inner.extend(other.inner);
        self.order.extend(other.order);
    }
}
