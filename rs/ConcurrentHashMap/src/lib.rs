use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

const INITIAL_NBUCKETS: usize = 1;

pub struct MyHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> MyHashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
        }
    }
}

impl<K, V> MyHashMap<K, V>
where
    K: Hash + Eq
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket = (hasher.finish() % self.buckets.len() as u64) as usize;
        let bucket = &mut self.buckets[bucket];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if *ekey == key {
                return Some(mem::replace(  evalue, value));
            }
        }
        bucket.push((key, value));
        None
    }

    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };
    }

}