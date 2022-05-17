use std::borrow::Borrow;

/* 
 * This function can handle both owned and borrowed s's.
 */
fn something<T: Borrow<u32>>(n: T) {
    println!("something2: {}", n.borrow());
}

fn main() {
    {
        let mut n = 18;
        something(n);
        something(&n);
        something(&mut n);
        // ...
        // ...
        // ...
        println!("main n: {}", n);
    }
}
