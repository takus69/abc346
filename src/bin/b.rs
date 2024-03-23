use proconio::input;

fn main() {
    input! {
        mut w: usize,
        mut b: usize,
    }
    while w >= 7 {
        w -= 7;
        b -= 5;
    }
    let combinations = vec![
        (0, 0), (1, 1), (1, 0), (0, 1),
        (2, 1), (2, 2), (1, 2), (2, 0),
        (3, 2), (3, 3), (3, 1), (2, 3),
        (4, 3), (4, 2), (4, 4), (5, 3),
        (5, 4), (5, 5), (6, 4), (6, 5),
        (6, 3), (5, 2),
    ];
    let mut ans = String::new();
    if combinations.iter().any(|&x| x == (w, b)) {
        ans = "Yes".to_string();
    } else {
        ans = "No".to_string();
    }
    println!("{}", ans);
}
