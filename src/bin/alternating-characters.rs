use std::io;
fn main() {
    let lines: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    for _ in range(0u32, lines) {
        let line = io::stdin().read_line().ok().unwrap().trim().to_string();
        println!("{}", run(line.as_slice()));
    }
}

fn run(line: &str) -> u32 {
    let mut count = 0u32;
    let mut k = String::new();
    for c in line.chars() {
        if k.len() != 0 && k.pop().unwrap() == c { count += 1; }
        k.push(c);
    }
    return count;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(3, super::run("AAAA"));
        assert_eq!(4, super::run("BBBBB"));
        assert_eq!(0, super::run("ABABABAB"));
        assert_eq!(0, super::run("BABABA"));
        assert_eq!(4, super::run("AAABBB"));
    }
}
