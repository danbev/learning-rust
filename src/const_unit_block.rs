const _: () = {
    struct Something;
};

const _: () = {
    // notice that this allowed and will not clash.
    struct Something;
};

fn main() {
    println!("free constant example");
    //let s = second.Something;
}
