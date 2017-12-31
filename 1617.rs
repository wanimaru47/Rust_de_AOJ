use std::io;

pub fn scan() -> String {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf).unwrap();

    buf
}

fn main() {
    loop {
        let s1 = scan();
        if s1 == ".\n" {
            break;
        }
        let s2 = scan();
        if s1 == s2 {
            println!("IDENTICAL");
            continue;
        }

        let s1: Vec<&str> = s1.split("\"").collect();
        let s2: Vec<&str> = s2.split("\"").collect();

        let mut count: i32 = 0;
        let mut other: i32 = 0;

        for i in 0..(s1.len()) {
            if s1.len() != s2.len() {
                other = 1;
                break;
            }
            if i % 2 == 0 {
                if s1[i].to_string() != s2[i].to_string() {
                    other += 1;
                }
                continue;
            }
            if s1[i].to_string() != s2[i].to_string() {
                count += 1;
            }
        }

        if other == 0 && count == 1 {
            println!("CLOSE");
        } else {
            println!("DIFFERENT");
        }
    }
}
