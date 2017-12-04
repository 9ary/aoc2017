#!/usr/bin/env run-cargo-script

use std::collections::HashMap;

const adjacents: &[(i32, i32)] = &[(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];

fn main() {
    let input = 347991;

    let mut spiral = HashMap::new();
    spiral.insert((0, 0), 1);

    let mut ring = 0;
    let (mut x, mut y) = (0, 0);
    'rings: loop {
        ring += 1;
        x += 1;
        y += 1;
        for side in 0..4 {
            for cell in 0..ring * 2 {
                match side {
                    0 => y -= 1,
                    1 => x -= 1,
                    2 => y += 1,
                    3 => x += 1,
                    _ => unreachable!(),
                }
                let mut acc = 0;
                for &(x1, y1) in adjacents {
                    if let Some(v) = spiral.get(&(x + x1, y + y1)) {
                        acc += v;
                    }
                }
                spiral.insert((x, y), acc);
                if acc > input {
                    println!("Solution: {}", acc);
                    break 'rings
                }
            }
        }
    }
}
