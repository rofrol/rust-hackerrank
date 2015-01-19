use std::io;
fn main() {
    let l: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let r: u32 = io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    println!("{}", run(l, r));
}

fn run(l: u32, r: u32) -> u32 {
    let mut max: u32 = 0;
    for i in (l..r + 1) {
        for j in (i..r + 1) {
            if i ^ j > max { max = i ^ j; }
        }
    }
    return max;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(7, super::run(10, 15));
    }
}
