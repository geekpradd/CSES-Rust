use std::io;
use std::cmp;

fn main(){
    let mut inp1 = String::new();

    io::stdin()
        .read_line(&mut inp1)
        .expect("errror in input");

    let n: usize = inp1.trim().parse().expect("error in input");

    let mut inp2 = String::new();

    io::stdin()
        .read_line(&mut inp2)
        .expect("errror in input");

    let array: Vec<i64> = inp2.split_whitespace().map(|x| x.parse().expect("parsing error")).collect();

    let mut dp = vec![vec![vec![0i64; 4]; n]; n];

    for i in 0..n {
        dp[i][i][0] = array[i];
        dp[i][i][1] = array[i];
        dp[i][i][2] = 0;
        dp[i][i][3] = 0;
    }
    for size in 1..n {
        for i in 0..n-size {
            dp[i][i+size][0] = cmp::max(array[i] + dp[i+1][i+size][2], array[i+size] + dp[i][i+size-1][2]);
            dp[i][i+size][1] = cmp::max(array[i] + dp[i+1][i+size][3], array[i+size] + dp[i][i+size-1][3]);
            if array[i] + dp[i+1][i+size][3] > array[i+size] + dp[i][i+size-1][3] {
                dp[i][i+size][2] = dp[i+1][i+size][0];
            } else {
                dp[i][i+size][2] = dp[i][i+size-1][0];
            }
            if array[i] + dp[i+1][i+size][2] > array[i+size] + dp[i][i+size-1][2] {
                dp[i][i+size][3] = dp[i+1][i+size][1];
            } else {
                dp[i][i+size][3] = dp[i][i+size-1][1];
            }
        }
    }

    println!("{}", dp[0][n-1][0]);
}