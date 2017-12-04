#!/usr/bin/env run-cargo-script

use std::collections::HashSet;
use std::io;

fn main() {
    let mut valid = 0;

    'phraseloop: loop {
        let mut passphrase = String::new();
        match io::stdin().read_line(&mut passphrase) {
            Ok(n) => {
                if n == 0 {
                    break
                }

                let mut wordset = HashSet::new();
                for word in passphrase.split_whitespace() {
                    let mut word = word.chars().collect::<Vec<char>>();
                    word.sort();
                    if !wordset.insert(word) {
                        continue 'phraseloop
                    }
                }
                valid += 1;
            },
            Err(error) => {
                eprintln!("error: {}", error);
                break
            },
        }
    }

    println!("Valid passphrases: {}", valid);
}
