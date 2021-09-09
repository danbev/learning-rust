enum Thing {
    Empty,
    Something(i32),
}

fn main() {
    println!("Enum example");
    let t = Thing::Something(22);
    if let Thing::Something(value) = t {
        println!("t was: {}", value);
    }

    match Thing::Empty {
        Thing::Something(value) => {
            println!("match Something: {}", value);
        }
        Thing::Empty => {
            println!("match Empty!");
        }
    }

    match t {
        _ => {
            println!("match anything!");
        }
    }

}
