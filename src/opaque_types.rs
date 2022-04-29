// With out this we get: `impl Trait` in type aliases is unstable
#![feature(type_alias_impl_trait)] 

trait Exposed {
    fn exposed(&self) -> () { 
        println!("Exposed::exposed");
    }
}

trait Hidden {
    fn hidden(&self) -> () { 
        println!("Hidden::hidden");
    }
}

struct Something {}
impl Exposed for Something {}
impl Hidden for Something {}

type SExposed = impl Exposed;

fn something() -> SExposed {
    Something{}
}

fn main() {
    // With only the following line we get an error:
    // could not find defining uses
    // Here we are saying that SExposed is a type that only 
    let s = something();
    s.exposed();
}
