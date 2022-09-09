use std::cell::UnsafeCell;

fn main() {
    let un = UnsafeCell::new(18);
    println!("un: {:?}", un);

    // into_inner returnes the wrapped/inner value:
    let value = un.into_inner();
    println!("value: {:?}", value);

    // Multiple *mut pointers are allowed:
    let un = UnsafeCell::new(18);
    let p1: *mut i32 = un.get();
    let p2: *mut i32 = un.get();
    println!("p1: {:?}, *p1: {}", p1, unsafe { *p1 });
    println!("p2: {:?}, *p2: {}", p2, unsafe { *p2 });

    // Only one &mut allowed:
    let mut mun = UnsafeCell::new(18);
    let r1: &mut i32 = mun.get_mut();
    println!("r1: {:?}", r1);

}
