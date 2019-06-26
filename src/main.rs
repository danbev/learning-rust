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
    println!("main...");
}

