trait Doit {
    fn doit(&self) -> ();
}

struct Something {
    id: i32
}

impl Doit for Something {
    fn doit(&self) -> () {
        println!("Something::doit...");
    }
}

static S: Something = Something {
    id: 10
};

fn main() {
    println!("Static example");
    <Something as crate::Doit>::doit(&S)
    
}
