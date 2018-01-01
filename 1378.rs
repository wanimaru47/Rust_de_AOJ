use std::io;
use std::str::FromStr;

pub fn scan() -> String {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf).unwrap();

    buf
}

fn main() {
    let s = scan();
    let s: Vec<&str> = s.split(" ").collect();
    let l = i32::from_str(s[0].trim()).unwrap();
    let k = i32::from_str(s[1].trim()).unwrap();

    let mut dp = vec![0 as i64; (l + 1) as usize];
    if 1 <= l {
        dp[1] = 1
    };
    if k <= l {
        dp[k as usize] = 1
    };

    let a = 1 + 1;
    let b = 1 + k;

    for i in 0..(l + 1) {
        if i >= a {
            dp[i as usize] += dp[(i - a) as usize];
        }
        if i >= b {
            dp[i as usize] += dp[(i - b) as usize];
        }
    }

    let mut sum: i64 = 0;
    for v in dp {
        sum += v;
    }

    println!("{}", sum);
}
