use sometrait::Something;

pub struct SomethingStruct{
}

impl Something for SomethingStruct {
    fn doit(&self) -> () {
        println!("SomethingImpl2: doit called");
    }
}
