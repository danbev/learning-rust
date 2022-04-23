trait Backend {
    fn process(&self, nr: i32) -> i32;
}

struct PositiveBackend {}
struct NegativeBackend {}

impl Backend for PositiveBackend {
    fn process(&self, nr: i32) -> i32 {
        nr
    }
}

impl Backend for NegativeBackend {
    fn process(&self, nr: i32) -> i32 {
        nr
    }
}

struct Service<T: Backend> {
    backend: T,
}

fn main() {
    let mut backends: Vec<Backend> = Vec::new();
    backends.push(PositiveBackend{});
    backends.push(NegativeBackend{});

}
