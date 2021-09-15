use typenum::{Integer, N4, Sum, P4};

fn main() {
    println!("typenum example");
    // pub type N4 = NInt<U4>;
    println!("N4 (Negetive 4): {}", N4::to_i32());

    type X = Sum<P4, P4>;
    println!("X: {}", X::to_i32());

}
