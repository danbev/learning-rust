#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
use std::fmt::Display;

trait Generic<T> {
    fn doit(&self, t: T) -> ();
}

trait Associated {
    type T;
    fn doit(&self, t: Self::T) -> ();
}

//trait GenericWithBound<T: Display> {
trait GenericWithBound<T> where T: Display {
    fn doit(&self, t: T) -> ();
}

trait AssociatedWithBound {
    type T: Display;
    fn doit(&self, t: Self::T) -> ();
}

// This is available thanks to the associated_type_defaults feature.
trait GenericWithDefault<T = u32> {
    fn doit(&self, t: T) -> ();
}

trait AssociatedWithDefault {
    type T = u32;
    fn doit2(&self, t: Self::T) -> ();
}

struct Something {}

impl AssociatedWithDefault for Something {
    fn doit2(&self, m: Self::T) {
        println!("AssociatedWithDefault doit. m: {}", m);
    }
}

trait GenericType {

    // Associated type which is unbound.
    type S1 = ();

    // Associated type which is bound.
    type Message<'m>: Sized where Self: 'm, = ();

    fn doit<'m>(&self, msg: Self::Message<'m>) -> ();
}

impl GenericType for Something {
    type Message<'m> = u32;

    fn doit<'m>(&self, msg: Self::Message<'m>) {
        println!("Something doit. m: {:#?}", msg);
    }
}

fn main() {
    println!("Type example");
    let s = Something{};
    let m = 18;
    s.doit(m);
    s.doit2(m);
}
