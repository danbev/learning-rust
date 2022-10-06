fn something(_: u32) -> Option<String> {
    Some(String::from("bajja"))
}

fn something_string(_: String) -> Option<String> {
    Some(String::from("string"))
}

fn main() {
    println!(
        "os2: {:?}",
        Some(18).and_then(something).and_then(something_string)
    );
    println!(
        "os3: {:?}",
        None.and_then(something).and_then(something_string)
    );

    let os = OptionalSomething {
        field: Some(Prov { x: 1 }),
    };

    let b = os.field.map_or(0, |p| p.x);
    println!("b: {}", b);
}

struct Prov {
    x: u8,
}

struct OptionalSomething {
    field: Option<Prov>,
}

impl OptionalSomething {
    fn has_prov(&self) -> u8 {
        self.field.as_ref().map_or(0, |p| p.x)
    }
}
