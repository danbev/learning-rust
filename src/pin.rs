use std::pin::Pin;

struct Something1<'a> {
    val: i32,
    ptr: &'a i32,
}

struct Something2<'a> {
    val: i32,
    ptr: Pin<&'a i32>,
}

#[derive(Debug)]
struct SomeUnpin {
    number: i32,
}

fn main() {
    let dummy = 10;
    let val = 8;
    let mut s = Something1{val: val, ptr: &dummy};
    s.ptr = &s.val;
    println!("&s.val: {:p}, &s.ptr: {:p}", &s.val, s.ptr);

    //let s2 = s; // cannot move out of s because it is borrowed.
    //println!("&s2.val: {:p}, &s2.ptr: {:p}", &s2.val, s2.ptr);
    let dummy2 = 11;
    let val2 = 9;
    let mut s2 = Something2{val: val, ptr: Pin::new(&dummy)};
    unsafe {
        s2.ptr = Pin::new_unchecked(&s2.val);
    }
    println!("&s2.val: {:p}, &s2.ptr: {:p}", &s2.val, s2.ptr);

    let n = SomeUnpin{ number: 18 };
    let n_pin = Pin::new(&n);
    println!("{:?}", n_pin);

    let p = Pin::new(&10);
    println!("{:?}", p);
}
