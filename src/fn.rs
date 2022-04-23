struct Something {
    y: i32,
    z: i32,
}

fn by_ref(input: &Something) {
}

fn by_val(intput: Something) {
}

fn main() {
    let x = Something{y: 3, z: 4};
    by_ref(&x);
    by_val(x);
}
