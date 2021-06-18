use std::io;
use std::convert::TryInto;

fn main(){
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let n: u32 = input.trim().parse().expect("Not an integer");

    let sum = (n*(n+1))/2;
    const MAXIMUM: usize = (500*501)/2;
    if sum % 2 == 1 {
        println!("0");
    } else {
        let value: usize = (sum/2).try_into().unwrap();
        let mut dp = vec![vec![0u32; MAXIMUM+1]; 501];
        const MOD: u32 = 1000000000 + 7;

        let siz: usize = n.try_into().unwrap();
        dp[0][0] = 1;
        for i in 1..siz+1 {

            dp[i][0] = 1;
            for j in 1..i {
                dp[i][j] = dp[i-1][j];
                // println!("{} {}", i, j);
                // println!("{}", dp[i][j]);
            }
            for j in i..value+1 {
                let remain:usize = j-i;
                dp[i][j] = (dp[i-1][j] + dp[i-1][remain])%MOD;
                // println!("{} {}", i, j);
                // println!("{}", dp[i][j]);
            }
        }
        let conv: i64 = dp[siz][value].into();
        let modconv: i64 = MOD.into();
        let ans = (500000004*conv)%modconv;
        println!("{}", ans);
    }
    

}