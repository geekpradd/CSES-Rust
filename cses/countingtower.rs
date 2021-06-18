use std::io;

fn main(){
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("input error");

    let t = tests.trim().parse().expect("parse error");

    let n: usize = 1000000;
    let mut dp = vec![0i64; n+1];
    let mut s = vec![0i64; n+1];

    s[0] = 1;
    dp[0] = 1;
    let mut dp_accum: i64 = 1;
    let mut s_accum: i64 = 1;
    const MOD: i64 = 1000000000 + 7;
    for i in 1..n+1 {
        s[i] = dp_accum;
        dp[i] = (s[i] + s_accum)%MOD;

        dp_accum = (dp_accum + dp[i])%MOD;
        s_accum = (s[i] + 4*s_accum)% MOD;
    }
    for _i in 0..t {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("input error");

        let c: usize = inp.trim().parse().expect("parse error");
        println!("{}", dp[c]);
    }
}