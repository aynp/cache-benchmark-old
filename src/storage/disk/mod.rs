pub struct Disk {
    storage: Vec<u64>,
}

impl Disk {
    pub fn new(size: usize) -> Disk {
        let storage: Vec<u64> = vec![0; size];
        return Disk { storage };
    }

    pub fn read(&self, index: usize) -> u64 {
        return self.storage[index];
    }

    pub fn write(&mut self, index: usize, data: u64) -> bool {
        self.storage[index] = data;
        return true;
    }
}
