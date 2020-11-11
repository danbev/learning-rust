#[link(name = "cfunctions", kind="dylib")]
extern {
  fn doit() -> ();
}

fn main() {
  println!("Example of calling a c library.");
  unsafe {
    doit();
  }
}
