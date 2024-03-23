use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = (k+1)*k/2;
    let uni: Vec<_> = a.into_iter().collect::<HashSet<_>>().into_iter().collect();
    for ai in uni.iter() {
        if ai <= &k {
            ans -= ai;
        }
    }
    println!("{}", ans);
}