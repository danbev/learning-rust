use std::marker::PhantomData;

struct PhantomStruct<B> {
    b: *const B,
    marker: PhantomData<B>,
}

fn main() {
    println!("Phantom Data example");
    let x = 18;
    let ps: PhantomStruct<u32> = PhantomStruct {
        b: &x,
        marker: PhantomData
    };
    unsafe {
        println!("ps.b: {:#?}", *ps.b);
    }
    println!("ps.marker: {:#?}", ps.marker);
}
