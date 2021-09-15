use generic_array::typenum::U3;
use generic_array::{ArrayLength, GenericArray};

struct Arr<N: ArrayLength<i32>> {
    data: GenericArray<i32, N> // this is the type of the data field
}

fn main() {
    let arr = Arr::<U3>{ data: GenericArray::default() };
    println!("arr.data: {:?}", arr.data);
    println!("arr.data[0]: {:?}", arr.data[0]);
}
