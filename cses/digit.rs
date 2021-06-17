use std::io;
use std::convert::TryInto;
fn main(){
    let mut q = String::new();
    io::stdin()
        .read_line(&mut q)
        .expect("Error");

    let q: i64 = q.trim().parse().expect("parse error");

    for _i in 0..q {
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Error");

        let num: i64 = num.trim().parse().expect("parse error");

        if num < 10 {
            println!("{}", num);
        } else {
            let mut start: i64 = 9;
            let mut prev: i64 = 9;

            let mut length = 0;
            for i in 2..19 {
                start = start*10;
                let temp = prev + i*start;
                if num <= temp {
                    length = i;
                    break;
                }

                prev = temp;
            }

            let residual = num - prev - 1;
            let completed = residual/length;
            let small = length - 1;
            let base = i64::pow(10, small.try_into().unwrap()) + completed;

            let leftover: usize = (residual - completed*length).try_into().unwrap();
            let converted: Vec<char> = base.to_string().chars().collect();

            println!("{}", converted[leftover]);
        }
    }


    
}