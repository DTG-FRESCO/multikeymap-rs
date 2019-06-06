use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use hashlike::HashLike;
use maplit::hashset;

#[derive(Debug)]
pub struct MultiKeyMap<K, SK, V>
where
    K: Hash + Eq + Clone,
    SK: Hash + Eq + Clone,
{
    store: HashMap<K, V>,
    idx: HashMap<SK, HashSet<K>>,
}

impl<K, SK, V> MultiKeyMap<K, SK, V>
where
    K: Hash + Eq + Clone,
    for<'a> &'a K: IntoIterator<Item = &'a SK>,
    SK: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        MultiKeyMap {
            store: HashMap::new(),
            idx: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let ret = self.store.insert(k.clone(), v);
        if ret.is_none() {
            for sk in &k {
                if self.idx.contains_key(&sk) {
                    self.idx.get_mut(&sk).unwrap().insert(k.clone());
                } else {
                    self.idx.insert(sk.clone(), hashset!(k.clone()));
                }
            }
        }
        ret
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let ret = self.store.remove(k);
        if ret.is_some() {
            for sk in k {
                let hs = self.idx.get_mut(&sk).unwrap();
                hs.remove(k);
                if hs.is_empty() {
                    self.idx.remove(&sk);
                }
            }
        }
        ret
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.store.contains_key(k)
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.store.get(k)
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.store.get_mut(k)
    }

    pub fn keys_containing(&self, k: &SK) -> Option<impl Iterator<Item = &K>> {
        self.idx.get(k).map(|v| v.iter())
    }

    pub fn mem_usage(&self) -> u64 {
        fn size_of_hm<K, V>(v: &HashMap<K, V>) -> u64 {
            use std::mem::size_of;

            ((v.capacity() * 11 / 10) * (size_of::<K>() + size_of::<V>() + size_of::<u64>())) as u64
        }
        size_of_hm(&self.store) + size_of_hm(&self.idx)
    }

    pub fn mem_utalisation(&self) -> u64 {
        fn size_of_hm<K, V>(v: &HashMap<K, V>) -> u64 {
            use std::mem::size_of;

            (v.len() * (size_of::<K>() + size_of::<V>() + size_of::<u64>())) as u64
        }
        size_of_hm(&self.store) + size_of_hm(&self.idx)
    }
}

impl<K, SK, V> HashLike<K, V> for MultiKeyMap<K, SK, V>
where
    K: Hash + Eq + Clone,
    for<'a> &'a K: IntoIterator<Item = &'a SK>,
    SK: Hash + Eq + Clone,
{
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.insert(k, v)
    }

    fn remove(&mut self, k: &K) -> Option<V> {
        self.remove(k)
    }

    fn contains_key(&self, k: &K) -> bool {
        self.contains_key(k)
    }

    fn get(&self, k: &K) -> Option<&V> {
        self.get(k)
    }

    fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.get_mut(k)
    }
}
