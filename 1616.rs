use std::io;
use std::str::FromStr;

pub fn scan() -> String {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf).unwrap();

    buf
}

fn main() {
    loop {
        let s = scan();
        let s: Vec<&str> = s.split(" ").collect();
        let n: i32 = i32::from_str(s[0].trim()).unwrap();
        let m: i32 = i32::from_str(s[1].trim()).unwrap();

        if n == 0 && m == 0 {
            break;
        }

        let s = scan();
        let s: Vec<&str> = s.split(" ").collect();

        let mut v: Vec<i32> = Vec::new();
        for i in 0..(s.len()) {
            v.push(i32::from_str(s[i].trim()).unwrap());
        }

        let mut sum = -1;
        for i in 0..(v.len()) {
            for j in 0..(v.len()) {
                if i == j {
                    continue;
                }
                if v[i] + v[j] <= m && v[i] + v[j] > sum {
                    sum = v[i] + v[j];
                }
            }
        }

        if sum < 0 {
            println!("NONE");
        } else {
            println!("{}", sum);
        }
    }
}
