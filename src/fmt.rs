fn main() {
    let name = "Fletch";
    // The following is using fmt named parameter, but in this case there is no
    // named parameter does not appear in the parameter list in which case the
    // parameter name will be referenced in the current scope.
    println!("name: {name}");
}
