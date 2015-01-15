use std::io;

fn main() {
    let num1: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let num2: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    println!("{}", num1 + num2);
}
