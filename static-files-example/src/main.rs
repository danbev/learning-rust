include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn main() {
    println!("static-files example...");
    let generated = generate();
    //println!("{generated:?}");
    let hello = generated.get("hello");
    println!("{:?}", std::str::from_utf8(hello.unwrap().data).unwrap());
}
