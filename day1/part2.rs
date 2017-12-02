#!/usr/bin/env run-cargo-script

use std::io::{self,Write};
use std::iter::Iterator;

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

    let mut digits1: Vec<char> = dstring.chars().collect();
    let len = digits1.len();
    if len % 2 != 0 {
        eprintln!("Got odd number of digits");
        return
    }
    let digits2 = digits1.split_off(len / 2);
    let pairs = digits1.iter().chain(digits2.iter())
        .zip(digits2.iter().chain(digits1.iter()));

    let mut sum = 0;
    for (a, b) in pairs {
        let d = *a as i32 - '0' as i32;
        if d < 0 || d > 9 {
            eprintln!("Non digit character '{}' found", a);
            return
        }
        if a == b {
            sum += d;
        }
    }

    println!("Sum: {}", sum);
}
