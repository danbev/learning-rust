const NR: u32 = 100;

#[cfg(target_os = "macos")]
fn printos() {
    println!("You are running macos!");
}

#[cfg(target_os = "linux")]
fn printos() {
    println!("You are running linux!");
}

#[cfg(target_os = "wasi")]
fn printos() {
    println!("You are running wasi!");
}

fn match_pattern(text: &'static str, pattern: &'static str) -> bool {
    println!("text={}, pattern={}", text, pattern);
    return false;
}

#[cfg(test)]
mod tests {
    #[test]
    fn match_test() {
        let text: &'static str = "bajja";
        let pattern: &'static str = "bajja";
        assert!(super::match_pattern(text, pattern) == false);
        assert!(super::match_pattern(text, pattern) == false);
    }
}

fn five() -> i32 {
    5
}

mod myns {
    pub fn ns_doit() {
        println!("ns_doit...");
    }
}

mod bajja;

fn main() {
    println!("main...NR={}", NR);
    let _shadowed = 10;
    let _shadowed = "bajja";
    let _shadowed = _shadowed.len();
    println!("shaddowed={}", _shadowed);
    printos();

    let tup: (i32, u8, f64) = (500, 8, 6.4);
    println!("typ={:?}", tup);
    println!("typ0={}", tup.0);

    let arr = [1, 2, 3];
    println!("arr={:?}", arr);
    let x = {
        5
    };
    println!("x={}", x);
    println!("five={}", five());
    myns::ns_doit();

    let danbev = Person {
        name: String::from("Daniel Bevenius")
    };
    Person::log();
    println!("{:?}", danbev);
    println!("{:#?}", danbev);
    println!("{}", danbev.go_berserk());
    println!("danbev={}", danbev.name);
    // panic();
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);

}

#[derive(Debug)]
struct Person {
    name: String
}

pub trait Berserk {
    fn go_berserk(&self) -> String {
        String::from("Something goes bererk!!")
    }

    fn log() {
        println!("Berserk!");
    }
}

impl Berserk for Person {
    fn go_berserk(&self) -> String {
        String::from("Person goes bererk!!")
    }
}

fn panic() -> ! {
    panic!("something panic...");
}
