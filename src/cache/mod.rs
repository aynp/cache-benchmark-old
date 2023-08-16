use crate::storage;

#[derive(Copy, Clone)]
struct Item {
    key: usize,
    value: u64,
}

pub struct Cache {
    data: Vec<Item>,
    disk: storage::Disk,
}

impl Cache {
    pub fn new(size: usize, disk: storage::Disk) -> Cache {
        let data: Vec<Item> = vec![Item{key: 0, value: 0}; size];
        return Cache {
            data,
            disk,
        }
    }

    pub fn read(&self, key: usize) -> u64 {
        match self.data
            .iter()
            .find(|&item| item.key == key) {
                Some(item) => item.value,
                None => self.disk.read(key)
            }
    }

    pub fn write(&mut self, key: usize, data: u64) -> bool {
        self.disk.write(key, data);
        return true;
    }
}

