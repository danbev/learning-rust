#[derive(Debug)]
struct Something {
}

#[derive(Debug)]
struct SomethingWithRef<'a> {
    ref_: &'a i32,
}

fn main() {
    let mut i = 5;
    let r: &i32 = &i;
    //println!("i: {:p} : {}", &i, i);
    //println!("r: {:p} : {}", &r, r);

    let rr: &&i32 = &r;

    i = 6;
    //println!("rr: {:p} : {}", &rr, (&&*rr));
    let s = Something{};
    give_up(s);
    //println!("Can't use s after give_up(s) {:?}", s);
    
    let mut s2 = Something{};
    read_only(&s2);
    mut_aswell(&mut s2);

    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    println!("Note that ref == ref follows the references: {}", rx == ry);
    println!("Use std::ptr::eq(ref, ref) to compare the addresses: {}", std::ptr::eq(rx,ry));

    let sr = SomethingWithRef{ref_: ry};
    println!("{:?}", sr);
}

fn give_up(s: Something) {
    println!("ownership taken by give_up function: {:?}", s);
}

fn read_only(s: &Something) {
    println!("read_only reference to s : {:?}", s);
}

fn mut_aswell(s: &mut Something) {
    println!("mut_aswell mut reference to s : {:?}", s);
}
