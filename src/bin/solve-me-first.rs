use std::old_io;

fn main() {
    let num1: u32 = old_io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    let num2: u32 = old_io::stdin().read_line().ok().unwrap().trim().parse().unwrap();
    println!("{}", num1 + num2);
}

fn run(l: u32, r: u32) -> u32 {
    return l + r;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(5, super::run(2, 3));
    }
}
