use arena::Arena;
use arena::Id;
fn main() {
    let mut x: Arena<u32> = Arena::new();
    x.alloc(10);
    x.alloc(12);
    x.alloc(13);
    x.alloc(15);
    let id = Id{index:2, generation:0};
    x.remove(&id);

    println!("Hello, world!");
}
