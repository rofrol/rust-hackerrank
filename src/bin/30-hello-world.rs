use std::io;
use std::io::prelude::*;

fn main() {
    let reader = io::stdin();
    let input: String = reader.lock().lines().next().unwrap().ok().unwrap();
    println!("{}", run(&input));
}

fn run(input: &str) -> String {
    format!("Hello, World.\n{}", input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // echo 'Welcome to 30 Days of Code!' | cargo test --bin 30-hello-world
        assert_eq!(
            "Hello, World.\nWelcome to 30 Days of Code! - cargo test",
            super::run("Welcome to 30 Days of Code! - cargo test")
        );
    }
}
