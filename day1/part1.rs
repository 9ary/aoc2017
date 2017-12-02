#!/usr/bin/env run-cargo-script

use std::io::{self,Write};

fn main() {
    print!("Digit string: ");
    io::stdout().flush();
    let mut dstring = String::new();
    match io::stdin().read_line(&mut dstring) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            return
        },
    }

    dstring.pop();
    let first = dstring.chars().nth(0).unwrap();
    dstring.push(first);

    let digits: Vec<char> = dstring.chars().collect();
    let mut sum = 0;
    for pair in digits.as_slice().windows(2) {
        let d = pair[0] as i32 - '0' as i32;
        if d < 0 || d > 9 {
            eprintln!("Non digit character '{}' found", pair[0]);
            return
        }
        if pair[0] == pair[1] {
            sum += d;
        }
    }

    println!("Sum: {}", sum);
}
