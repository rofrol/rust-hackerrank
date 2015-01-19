use std::io;
fn main() {
    let lines: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let mut num: u32;
    for _ in (0u32..lines) {
        num = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
        println!("{}", run(num));
    }
}

fn run(num: u32) -> u64 {
    let mut count: u64 = 1;
    for k in (1u32..num + 1) {
        count = if k % 2 == 0 {
            count + 1
        } else {
            count * 2
        };
    }
    return count;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(7, super::run(4));
        assert_eq!(6, super::run(3));
    }
}
