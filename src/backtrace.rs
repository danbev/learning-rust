fn a() {
    b()
}

fn b() {
    c()
}

fn c() {
    d()
}

fn d() {
    panic!()
}

fn main() {
    a()
}
