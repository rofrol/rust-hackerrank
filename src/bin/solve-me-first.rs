use std::io;

fn main() {
    let line1: String = io::stdin().read_line().unwrap();
    let line1_trimmed: &str = line1.as_slice().trim();
    let num1: int = line1_trimmed.parse::<int>().unwrap();
    let line2: String = io::stdin().read_line().unwrap();
    let line2_trimmed: &str = line2.as_slice().trim();
    let num2: int = line2_trimmed.parse::<int>().unwrap();
    print!("{}", num1 + num2);
}
