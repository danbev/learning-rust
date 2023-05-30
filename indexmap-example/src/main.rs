use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    String(String),
    Number(i32),
}

fn main() {
    let mut map = IndexMap::new();
    map.insert(Key::String("key".to_string()), 18);
    println!("map {:?}", map);
}
