use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut b = Vec::<usize>::new();

    let mut pre_a = a[0];
    for i in 1..n {
        b.push(pre_a*a[i]);
        pre_a = a[i];
    }
    println!("{}", b.iter().map(|&s| s.to_string()).collect::<Vec<String>>().join(" "));
}
