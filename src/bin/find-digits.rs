use std::io;
fn main() {
    let line = io::stdin().read_line().unwrap();
    let line_t: &str = line.trim();
    let count: int = line_t.parse::<int>().unwrap();
    let mut v: Vec<int> = Vec::new();
    for line in io::stdin().lock().lines() {
        let line_u: String = line.unwrap();
        let line_t: &str = line_u.as_slice().trim();
        let num: int = line_t.parse::<int>().unwrap();
        v.push(num);
    }
    let mut k: int;
    let mut count: int;
    for x in v.iter() {
        k = *x;
        count = 0;
        while k != 0 {
            // don't divide by 0 && don't count k < 10, ie. 3%(3%10) == 0
            if k % 10 != 0 && k >= 10 && k % (k % 10) == 0 {
                count += 1;
            }
            k = k/10;
        }
        println!("{}", count);
    }
}
