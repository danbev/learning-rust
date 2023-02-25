use memchr::memchr;

fn main() {
    let haystack = b"is this a dagger I see before me?";
    let needle = b'd';
    println!("{:?}", memchr(needle, haystack));
}
