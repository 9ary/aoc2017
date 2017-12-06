#!/usr/bin/env run-cargo-script

use std::io::{self,Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut jumps: Vec<isize> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut pc = 0;
    let mut jc = 0;

    loop {
        if let Some(j) = jumps.get_mut(pc) {
            pc += *j as usize;
            *j += 1;
            jc += 1;
        } else {
            break
        }
    }

    println!("{}", jc);
}
