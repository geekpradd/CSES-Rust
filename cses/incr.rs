use std::io;
use std::convert::TryFrom;

fn main(){
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Error");

    let n: u64 = n.trim().parse().expect("not integer");
    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

    let v: Vec<i64> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    let siz: usize = usize::try_from(n).unwrap();
    
    let mut base: i64 = v[0];
    let mut cost: i64  = 0;
    for i in 1..siz {
        if v[i] < base {
            cost += base - v[i];
        } else {
            base = v[i];
        }
    }
    println!("{}", cost);
}