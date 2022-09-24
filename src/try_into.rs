
struct Something {
    x: u32,
}

impl std::convert::TryInto<u32> for Something {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        Ok(self.x)
    }
}

#[derive(Clone, Copy)]
struct Something2 {
    x: u8,
}

fn main() {
   let s = Something { x: 18 }; 
   let x: u32 = s.try_into().unwrap();
   println!("x: {}", x);

   let b: [u8; 4] = [1, 2, 3, 4];

   // The following can be used to display the type:
   // let slice: () = &b[1..3];
   let slice: &[u8] = &b[1..3];
   println!("{:#?}", slice);

   // The following try_into call is being done on the [u8] which is an array
   // and implements TryFrom which implies TryInto.
   let v: &[u8; 2] = b[1..3].try_into().unwrap();
   println!("{:#?}", v);
}
