use std::mem::ManuallyDrop;

#[repr(C)]
union Something {
    s1: i32,
    s2: bool,
}

#[repr(C)]
union UntaggedOption<T> {
    none: (),
    some: ManuallyDrop<T>,
}

fn main() {
    let s = Something { s1: 1 };
    unsafe {
        println!("s1: {}", s.s1);
        println!("s2: {}", s.s2);
    }

    let mut untagged: UntaggedOption<i32> = UntaggedOption { none: () };
    unsafe {
        println!("untagged: {:?}", untagged.none);

        untagged.some = ManuallyDrop::new(18);
        println!("untagged: {:?}", untagged.some);
    }
}
