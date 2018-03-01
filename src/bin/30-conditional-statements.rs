use std::io;
use std::io::prelude::*;

fn main() {
    let reader = io::stdin();
    let num: u32 = reader
        .lock()
        .lines()
        .next()
        .unwrap()
        .ok()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    println!("{}", run(num));
}

fn run(num: u32) -> &'static str {
    if num % 2 != 0 || num % 2 == 0 && num >= 6 && num <= 20 {
        "Weird"
    } else {
        "Not Weird"
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("Weird", super::run(1));
        assert_eq!("Not Weird", super::run(2));
        assert_eq!("Weird", super::run(5));
        assert_eq!("Weird", super::run(6));
        assert_eq!("Weird", super::run(20));
        assert_eq!("Weird", super::run(21));
        assert_eq!("Not Weird", super::run(100));
    }
}
