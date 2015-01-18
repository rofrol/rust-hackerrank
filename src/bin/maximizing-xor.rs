use std::io;
fn main() {
    let l: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let r: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let mut max: u32 = 0;
    for i in range(l, r + 1) {
        for j in range(i, r + 1) {
            if i ^ j > max { max = i ^ j; }
        }
    }
    println!("{}", max);
}
