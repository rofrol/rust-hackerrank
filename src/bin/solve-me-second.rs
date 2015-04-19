use std::io;
use std::io::prelude::*;

fn main() {
    let reader = io::stdin();
    let lines: u32 = reader.lock().lines().next().unwrap().ok().unwrap().trim().parse().unwrap();
    for _ in (0u32..lines) {
        let line = reader.lock().lines().next().unwrap().ok().unwrap().trim().to_string();
        println!("{}", run(&line));
    }
}

fn run(line: &str) -> u64 {
    let mut num: u64 = 0;
    for item in line.split(' ') {
        num += item.parse::<u64>().unwrap();
    }
    return num;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(5, super::run("2 3"));
        assert_eq!(10, super::run("3 7"));
    }
}
