mod storage;
use storage::Storage;

fn main() {
    let mut storage = Storage::new(4, 10);

    storage.put(1, 10);
    let val = storage.get(1);

    println!("{}", val);
}
