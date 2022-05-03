// taken from https://cfsamson.github.io/books-futures-explained/4_generators_async_await.html
fn main() {
    let mut gen = GeneratorA::start();
    let mut gen2 = GeneratorA::start();

    if let GeneratorState::Yielded(n) = gen.resume() {
        println!("Got value {}", n);
    }

    // Is we include the following line the program will cause a segfault
    // because the pointer `borrowed` points to the field to_borrow which 
    // has now been swapped to a new location, but the borrowed pointer still
    // point to the old address, that are not updated.
    std::mem::swap(&mut gen, &mut gen2);

    if let GeneratorState::Yielded(n) = gen2.resume() {
        println!("Got value {}", n);
    }

    if let GeneratorState::Complete(()) = gen.resume() {
        ()
    };
}

enum GeneratorState<Y, R> {
    Yielded(Y),
    Complete(R),
}

trait Generator {
    type Yield;
    type Return;
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
}

enum GeneratorA {
    Enter,
    Yield1 {
        to_borrow: String,
        borrowed: *const String, // NB! This is now a raw pointer!
    },
    Exit,
}

impl GeneratorA {
    fn start() -> Self {
        GeneratorA::Enter
    }
}
impl Generator for GeneratorA {
    type Yield = usize;
    type Return = ();
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
            match self {
            GeneratorA::Enter => {
                let to_borrow = String::from("Hello");
                let borrowed = &to_borrow;
                let res = borrowed.len();
                *self = GeneratorA::Yield1 {to_borrow, borrowed: std::ptr::null()};

                // NB! And we set the pointer to reference the to_borrow string here
                if let GeneratorA::Yield1 {to_borrow, borrowed} = self {
                    *borrowed = to_borrow;
                }

                GeneratorState::Yielded(res)
            }

            GeneratorA::Yield1 {borrowed, ..} => {
                let borrowed: &String = unsafe {&**borrowed};
                println!("{} world", borrowed);
                *self = GeneratorA::Exit;
                GeneratorState::Complete(())
            }
            GeneratorA::Exit => panic!("Can't advance an exited generator!"),
        }
    }
}
