use derive_macro::Bajja;

#[derive(Bajja)]
struct MyStruct {
    my_string: String,
    my_enum: MyEnum,
    my_number: f64,
}

#[derive(Bajja)]
struct MyTupleStruct(u32, String, i8);

#[derive(Bajja)]
enum MyEnum {
    VariantA,
    VariantB,
}

#[derive(Bajja)]
union MyUnion {
    unsigned: u32,
    signed: i32,
}

fn main() {
    MyStruct::doit();
    MyTupleStruct::doit();
    MyEnum::doit();
    MyUnion::doit();
}

