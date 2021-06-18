use std::io;
use std::cmp;
fn main(){
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("input error");

    let data: Vec<usize> = tests.split_whitespace().map(|x| x.parse().expect("parse error")).collect();

    let mut dp = vec![vec![-1i64; 501]; 501];

    let mut copy: i64 = 0;
    for i in 1..501 {
        
        dp[i][1] = copy;
        copy = copy+1;
        for j in 2..501 {
            dp[i][j] = 100000000000;
            if j==i {
                dp[i][i] = 0;
            } else {
                for k in 1..j-1 {
                    dp[i][j] = cmp::min(dp[i][j], 1 + dp[i][k] + dp[i][j-k]);
                }
                for k in 1..i-1 {   
                    dp[i][j] = cmp::min(dp[i][j], 1 + dp[k][j] + dp[i-k][j]);
                }
            }
        }
    }

    println!("{}", dp[data[0]][data[1]]);

}