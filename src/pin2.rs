#![allow(dead_code, unused_imports)]

use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe {&*(self.b)}
    }
}

fn main() {
    let mut t = Test::new("first");
    t.init();
    println!("t.a: {:p} {}", &t.a, t.a);
    println!("t.b: {:p} {:p} {}", &t.b, t.b, unsafe{&*t.b});

    let mut t2 = Test::new("second");
    t2.init();
    println!();
    println!("t2.a: {:p} {}", &t2.a, t2.a);
    println!("t2.b: {:p} {:p} {}", &t2.b, t2.b, unsafe{&*t2.b});

    println!();
    println!("Now swap t and t2)");
    std::mem::swap(&mut t, &mut t2);

    println!();
    println!("t.a: {:p} {}", &t.a, t.a);
    println!("t.b: {:p} {:p} {}", &t.b, t.b, unsafe{&*t.b});

    println!();

    println!("t2.a: {:p} {}", &t2.a, t2.a);
    println!("t2.b: {:p} {:p} {}", &t2.b, t2.b, unsafe{&*t2.b});
}
