use std::io;
fn main() {
    let line_u = io::stdin().read_line().unwrap();
    let line_t: &str = line_u.trim();
    let lines: u64 = line_t.parse::<u64>().unwrap();
    let mut v: Vec<u64> = Vec::new();
    for i in range(0u64, lines) {
        let line_u = io::stdin().read_line().unwrap();
        let line_t: &str = line_u.as_slice().trim();
        let num: u64 = line_t.parse::<u64>().unwrap();
        v.push(num);
    }
    let mut k: u64;
    let mut count: u64;
    for x in v.iter() {
        k = *x;
        count = 0;
        while k != 0 {
            if k % 10 != 0 && *x % (k % 10) == 0 {
                count += 1;
            }
            k = k / 10;
        }
        println!("{}", count);
    }
}
