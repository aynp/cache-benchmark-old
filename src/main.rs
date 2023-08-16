mod storage;
use storage::Disk;

mod cache;
use cache::Cache;


fn main() {
    let disk = Disk::new(10);
    let mut cache = Cache::new(3, disk);

    cache.write(1, 10);

    println!("{}", cache.read(1));
}
