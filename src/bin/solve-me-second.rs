use std::io;
fn main() {
    let lines: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let mut num: u64;
    for i in range(0u32, lines) {
        let line = io::stdin().read_line().ok().unwrap().trim().to_string();
        num = 0;
        for item in line.as_slice().split(' ') {
            num += item.parse::<u64>().unwrap();
        }
        println!("{}", num);
    }
}
