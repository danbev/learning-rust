const NR: u32 = 100;

#[cfg(target_os = "macos")]
fn printos() {
    println!("You are running macos!");
}

#[cfg(linux)]
fn printos() {
    println!("You are running macos!");
}

fn match_pattern(text: &'static str, pattern: &'static str) -> bool {
    println!("text={}, pattern={}", text, pattern);
    return false;
}

#[test]
fn match_test() {
    let text: &'static str = "bajja";
    let pattern: &'static str = "bajja";
    assert!(match_pattern(text, pattern) == false);
}

fn main() {
    println!("main...NR={}", NR);
    printos();
}

