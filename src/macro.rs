macro_rules! expression_macro {
    (x => $e:expr) => ( println!("in some_macro...x: {}", $e));
    (y => $e:expr) => ( println!("in some_macro...y: {}", $e));
}

macro_rules! function_macro {
    () => (
        something0();
    );
    ($a1:expr) => (
        something1($a1);
    );
    ($a1:expr, $a2:expr) => (
        something2($a1, $a2);
    );
}

fn something0() {
    println!("something no args");
}

fn something1(input: i32) {
    println!("something1 function, input: {}", input);
}

fn something2(input1: i32, input2: i32) {
    println!("something1 function, input1: {}, input2: {}", input1, input2);
}

fn main() {
    //expression_macro!(e => 10); // no rules expected this token
    expression_macro!(x => 10);
    expression_macro!(y => 20);

    function_macro!();
    function_macro!(1);
    function_macro!(1, 2);
}
