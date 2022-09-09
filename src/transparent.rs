//#[repr(C)]
#[repr(transparent)]
struct Grams(f64);

//#[repr(C)]
#[repr(transparent)]
struct Millimeters(f64);

#[repr(C)]
struct Object {
    grams: Grams,
    millimeters: Millimeters ,
}

extern {
    fn doit(_: *mut Object);

    // double set_grams(double g);
    // Rust will pass g in an integer register, but the C function set_grams
    // expects g to be in a floating point register. So using repr(C) not only
    // affects the memory layout but also the ABI. If we instead use
    // repr(transparent) this tells Rust that the types are only for be layed
    // out in memory the same way but ignored for ABI purposes.
    fn set_grams(g : Grams) -> Grams ;
}

fn main() {

}
