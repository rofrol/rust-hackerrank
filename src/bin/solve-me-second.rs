use std::io;
fn main() {
    let lines: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let mut num: u64;
    for i in range(0u32, lines) {
        let line_u = io::stdin().read_line().unwrap();
        let line_t = line_u.as_slice().trim();
        num = 0;
        for item in line_t.as_slice().split(' ') {
            num += item.parse::<u64>().unwrap();
        }
        println!("{}", num);
    }
}
