use std::io;
fn main() {
    let lines: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let mut k: u64;
    let mut num: u64;
    let mut count: u64;
    for i in range(0u32, lines) {
        num = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
        k = num;
        count = 0;
        while k != 0 {
            if k % 10 != 0 && num % (k % 10) == 0 {
                count += 1;
            }
            k = k / 10;
        }
        println!("{}", count);
    }
}
