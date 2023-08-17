mod cache;
use cache::Cache;

mod disk;
use disk::Disk;

pub struct Storage {
    cache: Cache,
    disk: Disk,
}

impl Storage {
    pub fn new(cache_size: usize, disk_size: usize) -> Storage {
        let lru = cache::EvictionStrategy::new(cache_size);
        return Storage {
            cache: Cache::new(lru),
            disk: Disk::new(disk_size),
        };
    }

    pub fn get(&mut self, key: usize) -> u64 {
        if let Some(value) = self.cache.get(key) {
            value
        } else {
            let value = self.disk.read(key);
            self.cache.insert(key, value);
            return value;
        }
    }

    pub fn put(&mut self, key: usize, value: u64) -> bool {
        self.cache.insert(key, value);
        self.disk.write(key, value);
        true
    }
}
