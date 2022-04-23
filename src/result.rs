
#[derive(Debug)]
enum SomeError {
    First,
    Second,
}

fn doit(input: u32) -> Result<u32, SomeError> {
    if input == 1 {
        Ok(input)
    } else {
        Err(SomeError::First)
    }
}

fn main() {
    let x: Result<u32, SomeError> = doit(1);
    //println!("{}", x.unwrap());
    x.map(|i|  println!("i = {}", i));

    let r: Result<usize, &'static str> = Ok(1);
    r.map_err(|_| SomeError::Second).and_then(|n: usize| {
        println!("in and_then: {}", n); 
        Ok(n)
    });
    println!("{:?}", r);

}
