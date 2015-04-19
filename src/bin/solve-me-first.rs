use std::io;
use std::io::prelude::*;

fn main() {
    let reader = io::stdin();
    let num1: u32 = reader.lock().lines().next().unwrap().ok().unwrap().trim().parse().unwrap();
    let num2: u32 = reader.lock().lines().next().unwrap().ok().unwrap().trim().parse().unwrap();
    println!("{}", num1 + num2);
}

fn run(l: u32, r: u32) -> u32 {
    return l + r;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(5, super::run(2, 3));
    }
}
