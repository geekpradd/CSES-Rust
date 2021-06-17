use std::io;
use std::convert::TryInto;
use std::collections::HashMap;

fn main(){
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("io error");
    
    let v: Vec<i32> = inp.split_whitespace().map(|x| x.parse().expect("parse error")).collect();

    let mut second = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("io error");
    
    let numbers: Vec<i32> = second.split_whitespace().map(|x| x.parse().expect("parse error")).collect();


    let refer: usize = v[0].try_into().unwrap();

    let mut holder = HashMap::new();

    let mut got = false;

    for i in 0..refer {
        for j in i+1..refer {
            let current = numbers[i] + numbers[j];

            if holder.contains_key(&current) {  
                println!("{} {} {}", holder[&current]+1, i+1, j+1);
                got = true;
                break;
            }
        }
        if got{
            break;
        }

        if !holder.contains_key(&(v[1] - numbers[i])){
            holder.insert(v[1] - numbers[i], i);
        }
    }
    if !got {
        println!("IMPOSSIBLE");
    }



}