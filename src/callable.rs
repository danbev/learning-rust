#![feature(unboxed_closures)]
#![feature(fn_traits)]

struct Something {
    s: String,
}

impl Fn<(String,)> for Something {
    extern "rust-call" fn call(&self, _args: (String,)) -> Self::Output {
        println!("Call (Fn) for Something: {}", _args.0);
        1
    }
}

impl FnMut<(String,)> for Something {
    extern "rust-call" fn call_mut(&mut self, _args: (String,)) -> Self::Output {
        println!("Call (FnMut) for Something {}", _args.0);
        self.s = _args.0;
        2
    }
}

impl FnOnce<(String,)> for Something {
    type Output = i32;

    extern "rust-call" fn call_once(self, _args: (String,)) -> Self::Output {
        println!("Call (FnOnce) for Something {}", _args.0);
        3
    }
}

fn main() {
    let x = Something {
        s: "Bajja".to_string(),
    };
    let r = x("Hello".to_string());
    println!("Result: {}", r);

    let r = x.call(("Hello".to_string(),));
    println!("Result: {}", r);

    let mut x = Something {
        s: "Bajja".to_string(),
    };
    x.call_mut(("Hello".to_string(),));
}
