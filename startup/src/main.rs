use libc::atexit;

fn main() {
    println!("main...");
    unsafe {
        atexit(at_exit_function);
    }
}

extern "C" fn at_exit_function() {
    println!("at_exit_function...");
    let _x = 100;
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".ctors")]
static CTOR: extern fn() = {
   extern fn ctor() {
       println!(".ctor...");
   }
   ctor
};

#[used]
//#[cfg_attr(target_os = "linux", link_section = ".dtors")]
#[cfg_attr(target_os = "linux", link_section = ".fini_array")]
static DTOR: extern fn() = {
   extern fn dtor() {
       println!(".dtor...");
   }
   dtor
};

