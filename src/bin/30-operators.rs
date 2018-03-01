use std::io;
use std::io::prelude::*;

fn main() {
    let reader = io::stdin();
    let num1: f64 = reader
        .lock()
        .lines()
        .next()
        .unwrap()
        .ok()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let num2: i64 = reader
        .lock()
        .lines()
        .next()
        .unwrap()
        .ok()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let num3: i64 = reader
        .lock()
        .lines()
        .next()
        .unwrap()
        .ok()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    println!("{}", run(num1, num2, num3));
}

fn run(num1: f64, num2: i64, num3: i64) -> String {
    format!(
        "The total meal cost is {} dollars.",
        (num1 + num2 as f64 / 100.0 * num1 as f64 + num3 as f64 / 100.0 * num1).round()
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // echo -e "12.00\n20\n8" | cargo run --bin 30-operators
        assert_eq!(
            "The total meal cost is 15 dollars.",
            super::run(12.00, 20, 8)
        );
        // echo -e "12.00\n22\n8" | cargo run --bin 30-operators
        assert_eq!(
            "The total meal cost is 16 dollars.",
            super::run(12.00, 22, 8)
        );
    }
}
