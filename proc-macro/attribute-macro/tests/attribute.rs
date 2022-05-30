#[attribute_macro::attribute_macro]
fn wrapped_function() {
    println!("wrapped_function...");
}

#[test]
fn test_attribute_macro() {
    wrapped_function();
}
