use std::ffi::CString;
use std::os::raw::c_char;

// The below can be left out if the library is specified to 
// rustc as an option.
//#[link(name = "cfunctions", kind="dylib")]
extern "C" {
  fn doit(nr: i32) -> ();
  fn print_string(s: *const c_char) -> ();
}

fn main() {
  println!("Example of calling a c library.");
  let s = CString::new("bajja").expect("CString::new failed");
  unsafe {
    doit(18);
    print_string(s.as_ptr());
  }
}
