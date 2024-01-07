use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait IterUtils: Iterator {
    fn collect_map<K, V>(self) -> HashMap<K, V>
    where
        HashMap<K, V>: FromIterator<Self::Item>,
        K: Hash + Eq,
        Self: std::marker::Sized,
    {
        self.collect::<HashMap<K, V>>()
    }
    fn collect_set<K>(self) -> HashSet<K>
    where
        HashSet<K>: FromIterator<Self::Item>,
        K: Hash + Eq,
        Self: std::marker::Sized,
    {
        self.collect::<HashSet<K>>()
    }
}

impl<T> IterUtils for T where T: Iterator {}
