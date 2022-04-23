use std::mem;

struct Something {
    callback: *const ()
}

impl Something {
    fn cb(&self) -> () {
        println!("Something::doit");
    }
}

fn main() {
  let callback = || => { println!("callback") };
  let s = Something{};
  let f: fn(*mut ()) = unsafe { mem::transmute(s.callback) };
}
