use id_arena::{Arena, Id};

#[derive(Debug, Clone)]
struct Something {
    name: String,
    age: u32,
}

fn main() {
    let mut arena: Arena<Something> = Arena::new();

    let idx: Id<Something> = arena.alloc(Something {
        name: "Fletch".to_string(),
        age: 48,
    });
    println!("idx: {:?}", idx);

    let value = arena.get(idx);
    println!("value: {:?}", value);

    let value = &arena[idx];
    println!("value: {:?}", value);
}
