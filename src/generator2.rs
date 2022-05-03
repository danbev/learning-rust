// taken from https://cfsamson.github.io/books-futures-explained/4_generators_async_await.html
fn main() {
    let mut gen = GeneratorA::start(4);

    if let GeneratorState::Yielded(n) = gen.resume() {
        println!("Got value {}", n);
    }

    if let GeneratorState::Complete(()) = gen.resume() {
        ()
    };
}

// If you've ever wondered why the parameters are called Y and R the naming from
// the original rfc most likely holds the answer
enum GeneratorState<Y, R> {
    Yielded(Y),  // originally called `Yield(Y)`
    Complete(R), // originally called `Return(R)`
}

trait Generator {
    type Yield;
    type Return;

    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
}

enum GeneratorA {
    Enter(i32),
    Yield1(i32),
    Exit,
}

impl GeneratorA {
    fn start(a1: i32) -> Self {
        GeneratorA::Enter(a1)
    }
}

impl Generator for GeneratorA {
    type Yield = i32;
    type Return = ();
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
        // lets us get ownership over current state
        match std::mem::replace(self, GeneratorA::Exit) {
            GeneratorA::Enter(a1) => {
                /*----code before yield----*/
                println!("Hello");
                let a = a1 * 2;

                *self = GeneratorA::Yield1(a);
                println!("resume done.");
                GeneratorState::Yielded(a)
            }

            GeneratorA::Yield1(_) => {
                /*-----code after yield-----*/
                println!("world!");

                *self = GeneratorA::Exit;
                println!("resume done.");
                GeneratorState::Complete(())
            }
            GeneratorA::Exit => {
                println!("resume done.");
                panic!("Can't advance an exited generator!")}
            ,
        }
    }
}
