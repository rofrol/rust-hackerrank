use std::io;

fn main() {
    let line1: String = io::stdin().read_line().unwrap();
    let line1_trimmed: &str = line1.as_slice().trim();
    let num1: int = from_str(line1_trimmed).unwrap();
    let line2: String = io::stdin().read_line().unwrap();
    let line2_trimmed: &str = line2.as_slice().trim();
    let num2: int = from_str(line2_trimmed).unwrap();
    //println! adds new line
    //println!("{}", num1 + num2);
    //but print! doesn't print anything
    //print!("{}", num1 + num2);
    let n = (num1 + num2).to_string();
    let p = n.as_slice();
    io::stdout().write(p.as_bytes());
}
