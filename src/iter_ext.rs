use std::collections::HashMap;
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
}

impl<T> IterUtils for T where T: Iterator {}
