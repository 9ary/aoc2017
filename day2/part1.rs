#!/usr/bin/env run-cargo-script

use std::io::{self,Read};

fn main() {
    let mut spreadsheet = String::new();
    io::stdin().read_to_string(&mut spreadsheet).unwrap();

    let mut checksum = 0;
    for line in spreadsheet.lines() {
        let cells: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
        checksum += cells.iter().max().unwrap() - cells.iter().min().unwrap();
    }

    println!("Checksum: {}", checksum);
}
