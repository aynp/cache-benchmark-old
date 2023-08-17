use crate::cache::lru::LRU;

pub mod lru;

pub trait EvictionStrategy {
    fn new(capacity: usize) -> Self;
    fn insert(&mut self, key: usize, value: u64);
    fn get(&mut self, key: usize) -> u64;
}

/*
 * TODO: Somehow pass eviction_strategy as parameter
pub enum EvictionType {
    LRU(LRU)
}
*/

pub struct Cache {
    eviction_strategy: LRU,
}

impl Cache {
    pub fn new(eviction_strategy: LRU) -> Self {
        Cache {
            eviction_strategy,
        }
    }

    pub fn insert(&mut self, key: usize, value: u64) {
        self.eviction_strategy.insert(key, value);
    }

    pub fn get(&mut self, key: usize) -> u64 {
        self.eviction_strategy.get(key)
    }
}

