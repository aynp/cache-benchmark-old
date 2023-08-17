mod storage;
use storage::Disk;

mod cache;
use cache::Cache;
use cache::lru::LRU;

use crate::cache::EvictionStrategy;


fn main() {
    let _disk = Disk::new(10);
    let lru = LRU::new(3);
    let mut cache = Cache::new(lru);

    cache.insert(1, 10);

    println!("{}", cache.get(1));
}

