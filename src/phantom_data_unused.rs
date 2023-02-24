struct Something<T, U> {
    first: T,
    _marker: std::marker::PhantomData<U>,
}

fn main() {}
