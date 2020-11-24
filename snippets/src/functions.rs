pub fn divergent() -> ! {
    loop {
        println!("looping...!");
        std::process::exit(10);
    }
}

#[cfg(test)]
mod test {
    pub fn test_divergent() {
        super::divergent();
    }
}
