use std::ops::Index;

#[derive(Debug)]
struct Test {
    data: Vec<u8>,
}

impl Test {
    fn new(data: Vec<u8>) -> Self {
        Test { data }
    }
}

impl<Idx> std::ops::Index<Idx> for Test
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        println!("Test.index...");
        &self.data[index]
    }
}

fn main() {
    let test = Test::new(vec![1, 2, 3]);

    //let slice = &test[1..];
    let slice = test.index(1..);
    println!("slice: {slice:?}");
    assert_eq!(slice, [2, 3]);

    let first = &test[0];
    println!("first: {first:?}");
    assert_eq!(first, &1);
}
