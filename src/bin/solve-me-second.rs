use std::old_io;
fn main() {
    let lines: u32 = old_io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    for i in (0u32..lines) {
        let line = old_io::stdin().read_line().ok().unwrap().trim().to_string();
        println!("{}", run(line.as_slice()));
    }
}

fn run(line: &str) -> u64 {
    let mut num: u64 = 0;
    for item in line.as_slice().split(' ') {
        num += item.parse::<u64>().unwrap();
    }
    return num;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(5, super::run("2 3"));
        assert_eq!(10, super::run("3 7"));
    }
}
