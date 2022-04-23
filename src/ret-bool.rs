fn something(data: &[u8; 2]) -> bool {
    data[0] & 0b11000000 == 0b10000000
}

fn main() {
    println!("Return bool example..");
    let data: [u8; 2] = [128, 0]; 
    let b = something(&data);
    println!("b: {}", b);

}

