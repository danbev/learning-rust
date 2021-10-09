struct UnitStruct;

trait Doit {
    fn print(&self);
}

impl Doit for UnitStruct {
    fn print(&self) {
        println!("Doit for UnitStruct...");
    }
}

struct AnotherUnitStruct();

fn main() {
    let u = UnitStruct;
    u.print();
    let _a = AnotherUnitStruct;
}
