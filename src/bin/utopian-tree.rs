use std::io;
fn main() {
    let lines: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let mut num: u32;
    let mut count: u64;
    for _ in range(0u32, lines) {
        num = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
        count = 1;
        for k in range(1u32, num + 1) {
            count = if k % 2 == 0 {
                count + 1
            } else {
                count * 2
            };
        }
        println!("{}", count);
    }
}
