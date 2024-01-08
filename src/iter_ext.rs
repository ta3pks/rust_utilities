use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use futures::{Future, StreamExt, TryStream, TryStreamExt};

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
// #[cfg(feature = "rayon")]
// use rayon::prelude::*;
#[cfg(feature = "rayon")]
pub trait ParIterUtils: rayon::iter::ParallelIterator {
    fn collect_map<K, V>(self) -> HashMap<K, V>
    where
        HashMap<K, V>: rayon::iter::FromParallelIterator<Self::Item>,
        K: Hash + Eq,
        Self: std::marker::Sized,
    {
        self.collect::<HashMap<K, V>>()
    }
    fn collect_set<K>(self) -> HashSet<K>
    where
        HashSet<K>: rayon::iter::FromParallelIterator<Self::Item>,
        K: Hash + Eq,
        Self: std::marker::Sized,
    {
        self.collect::<HashSet<K>>()
    }
    fn collect_vec<K>(self) -> Vec<K>
    where
        Vec<K>: rayon::iter::FromParallelIterator<Self::Item>,
        Self: std::marker::Sized,
    {
        self.collect::<Vec<K>>()
    }
}

#[cfg(feature = "rayon")]
impl<T> ParIterUtils for T where T: rayon::iter::ParallelIterator {}

pub trait AsyncIterUtils: futures::Stream {
    fn collect_vec(self) -> impl Future<Output = Vec<Self::Item>>
    where
        Self: Sized,
    {
        self.collect::<Vec<Self::Item>>()
    }
    fn collect_set<K>(self) -> impl Future<Output = HashSet<K>>
    where
        HashSet<K>: Extend<Self::Item>,
        K: Hash + Eq,
        Self: Sized,
    {
        self.collect::<HashSet<K>>()
    }
    fn collect_map<K, V>(self) -> impl Future<Output = HashMap<K, V>>
    where
        HashMap<K, V>: Extend<Self::Item>,
        K: Hash + Eq,
        Self: Sized,
    {
        self.collect::<HashMap<K, V>>()
    }
    fn try_collect_vec<E>(self) -> impl Future<Output = Result<Vec<Self::Item>, E>>
    where
        Self: Sized + TryStream<Error = E>,
        Vec<Self::Item>: Extend<Self::Ok>,
    {
        self.try_collect::<Vec<Self::Item>>()
    }
    fn try_collect_set<K, E>(self) -> impl Future<Output = Result<HashSet<K>, E>>
    where
        Self: Sized + TryStream<Error = E>,
        HashSet<K>: Extend<Self::Ok>,
        K: Hash + Eq,
    {
        self.try_collect::<HashSet<K>>()
    }
    fn try_collect_map<K, V, E>(self) -> impl Future<Output = Result<HashMap<K, V>, E>>
    where
        Self: Sized + TryStream<Error = E>,
        HashMap<K, V>: Extend<Self::Ok>,
        K: Hash + Eq,
    {
        self.try_collect::<HashMap<K, V>>()
    }
}

impl<T> AsyncIterUtils for T where T: futures::Stream {}
