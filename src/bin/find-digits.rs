use std::io;
fn main() {
    let lines: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let mut num: u64;
    for i in range(0u32, lines) {
        num = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
        println!("{}", run(num));
    }
}

fn run(num: u64) -> u64 {
    let mut k: u64 = num;
    let mut count: u64 = 0;
    while k != 0 {
        if k % 10 != 0 && num % (k % 10) == 0 {
            count += 1;
        }
        k = k / 10;
    }
    return count;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2, super::run(12));
        assert_eq!(3, super::run(1012));
    }
}
