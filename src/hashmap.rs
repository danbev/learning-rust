#![feature(type_name_of_val)]

use std::collections::HashMap;
use std::format;
use std::any::type_name_of_val;

fn main() {
    let mut map: HashMap<usize, String> = HashMap::new();
    let e = map.entry(1);
    println!("{:?}", e);
    // if the entry does not exist then insert the value:
    map.entry(1).or_insert(String::from("bajja"));

    let e = map.entry(1);
    println!("{:?}", e);

    let s  = format!("one {}", 20);
    println!("{}", type_name_of_val(&s));
    map.entry(2).or_insert_with_key(|k| format!("bajja_{}", k) );
    println!("{}", map.get(&2).unwrap());

}
