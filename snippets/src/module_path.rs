pub mod something {
    pub fn s() {
        println!("module_path: {:?}", module_path!());
    }
}

mod something_private {
    pub fn private_function() {
        print_something();
    }
    fn print_something() {
        println!("something_private...");
    }
}

#[cfg(test)]
mod tests {
    use crate::module_path::something_private::private_function;
    use crate::module_path::something_private::private_function as bajja;
    #[test]
    fn module_path() {
        crate::module_path::something::s();
        private_function();
        bajja();
    }
}

