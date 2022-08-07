trait FirstName {
    fn first(self: &Self) -> &'static str;
}

trait LastName {
    fn last(self: &Self) -> &'static str;
}

trait SuperTrait: FirstName + LastName {
    fn print_name(self: &Self) {
        println!("first name: {}, last name: {}", self.first(), self.last());
    }
}

struct Something {
    first: &'static str,
    last: &'static str,
}

impl FirstName for Something {
    fn first(self: &Self) -> &'static str {
        self.first
    }
}

impl LastName for Something {
    fn last(self: &Self) -> &'static str {
        self.last
    }
}

impl SuperTrait for Something {
}

fn main() {
    let s = Something{ first: "Muhatma", last: "Fletch"};
    s.print_name();
}
