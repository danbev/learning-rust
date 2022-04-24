use std::mem;
use std::mem::MaybeUninit;

struct Something<'a> {
    name: &'a str,
}

fn main() {
    println!("std::mem examples");
    // Can't call the following function, well we can but it will panic
    // saying 'attempting to zero-initialize type &i32 which is invalid'
    //let x: &i32 = unsafe { mem::zeroed() };
    //println!("{}", x);
    let x: i32  = unsafe { mem::zeroed() };
    println!("{}", x);

    let mut un_s = MaybeUninit::<Something>::uninit();

    let s = Something{name: "Fletch"};
    un_s.write(s);

    unsafe {
        println!("s.name: {}", (*un_s.as_ptr()).name);
    }


    unsafe {
        let s2 = un_s.assume_init();
        println!("s2.name: {}", s2.name);
    }


}
