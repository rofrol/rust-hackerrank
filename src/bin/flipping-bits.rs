use std::io;
use std::io::prelude::*;

fn main() {
    let mut reader = io::stdin();
    let lines: u32 = reader.lock().lines().next().unwrap().ok().unwrap().trim().parse().unwrap();
    let mut num: u32;
    for i in (0u32..lines) {
        num = reader.lock().lines().next().unwrap().ok().unwrap().trim().parse().unwrap();
        println!("{}", run(num));
    }
}

fn run(l: u32) -> u32 {
    return !l;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2147483648, super::run(2147483647));
        assert_eq!(4294967294, super::run(1));
        assert_eq!(4294967295, super::run(0));
    }
}
