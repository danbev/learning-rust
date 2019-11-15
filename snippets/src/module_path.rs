pub mod something {
    pub fn s() {
        println!("module_path: {:?}", module_path!());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn module_path() {
        crate::module_path::something::s();
    }
}

