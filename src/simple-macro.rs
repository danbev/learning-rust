macro_rules! add {
    ($a:expr,$b:expr) => {{
        $a + $b
    }};
}

fn main() {
    add!(1, 2);
}
