#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::min;
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }
    let mut dp = vec![1 << 60; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = min(dp[i], dp[i - 1] + (a[i] - a[i - 1]).abs());
        if i > 1 {
            dp[i] = min(dp[i], dp[i - 2] + (a[i] - a[i - 2]).abs());
        }
    }
    println!("{}", dp[n - 1]);
}
