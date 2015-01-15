use std::io;
fn main() {
    let lines: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let mut num: u32;
    for i in range(0u32, lines) {
        num = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
        println!("{}", !num);
    }
}
