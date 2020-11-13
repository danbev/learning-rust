// The below can be left out if the library is specified to 
// rustc as an option.
//#[link(name = "cfunctions", kind="dylib")]
extern "C" {
  fn doit(nr: i32) -> ();
}

fn main() {
  println!("Example of calling a c library.");
  unsafe {
    doit(18);
  }
}
