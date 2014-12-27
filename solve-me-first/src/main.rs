use std::io;

fn main() {
    let line1: String = io::stdin().read_line().unwrap();
    let line1_trimmed: &str = line1.as_slice().trim();
    let num1: int = line1_trimmed.parse::<int>().unwrap();
    let line2: String = io::stdin().read_line().unwrap();
    let line2_trimmed: &str = line2.as_slice().trim();
    let num2: int = line2_trimmed.parse::<int>().unwrap();
    //println! adds new line
    //println!("{}", num1 + num2);
    //but print! doesn't print anything
    //print!("{}", num1 + num2);
    let n = (num1 + num2).to_string();
    let p = n.as_slice();
    io::stdout().write(p.as_bytes());
}
