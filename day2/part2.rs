#!/usr/bin/env run-cargo-script

use std::io::{self,Read};

fn main() {
    let mut spreadsheet = String::new();
    io::stdin().read_to_string(&mut spreadsheet).unwrap();

    let mut checksum = 0;
    for line in spreadsheet.lines() {
        let mut cells: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
        // Sort in reverse, courtesy of Rust docs
        cells.sort_by(|a, b| b.cmp(a));
        'line: for (i, cell) in cells.iter().enumerate() {
            for cell2 in &cells[i + 1..] {
                if cell % cell2 == 0 {
                    checksum += cell / cell2;
                    break 'line
                }
            }
        }
    }

    println!("Checksum: {}", checksum);
}
