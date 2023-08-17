use std::collections::{HashMap, VecDeque};
use crate::storage::cache::EvictionStrategy;

pub struct LRU {
    cache: HashMap<usize, u64>,
    order: VecDeque<usize>,
    capacity: usize,
}

impl EvictionStrategy for LRU {
    fn new(capacity: usize) -> LRU {
        return LRU{
            cache: HashMap::with_capacity(capacity),
            order: VecDeque::new(),
            capacity,
        }
    }

    fn insert(&mut self, key: usize, value: u64) {
        if self.cache.len() >= self.capacity {
            self.order.pop_back();
        }
        self.cache.insert(key.clone(), value);
        self.order.push_front(key);
    }

    fn get(&mut self, key: usize) -> Option<u64> {
        if let Some(value) = self.cache.get(&key) {
            if let Some(position) = self.order.iter().position(|&k| k == key) {
                self.order.remove(position);
            }
            self.order.push_front(key.clone());
            Some(*value)
        } else {
            None
        }
    }
}

