#!/usr/bin/env run-cargo-script

use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut state: Vec<i32> = input.split_whitespace().map(|b| b.parse().unwrap()).collect();
    let mut seen_states = HashSet::new();

    let mut count = 0;
    loop {
        let (mut bank, &blocks) = state.iter().enumerate().rev().max_by_key(|&(_, b)| b).unwrap();

        state[bank] = 0;
        for _ in 0..blocks {
            bank = (bank + 1) % state.len();
            state[bank] += 1;
        }

        count += 1;

        if !seen_states.insert(state.clone()) {
            break;
        }
    }

    println!("{}", count);
}
