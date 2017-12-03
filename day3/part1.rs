#!/usr/bin/env run-cargo-script

use std::cmp;

fn main() {
    let input = 347991;

    // We split the spiral in rings, the 1 in the middle is ring 0.
    // For all N > 0, ring N has N * 8 numbers.
    // Ring N ends at N * (N + 1) * 4 + 1 = 4N^2 + 4N + 1.
    // Now we can find which ring the input is in by solving a trivial quadratic equation.

    let a = 4;
    let b = 4;
    let c = 1 - input;

    let delta = (b * b - 4 * a * c) as f64;
    let root1 = (-b as f64 - delta.sqrt()) / (2 * a) as f64;
    let root2 = (-b as f64 + delta.sqrt()) / (2 * a) as f64;

    let ring = cmp::max(root1 as i32, root2 as i32) + 1;
    println!("Input is in ring {}", ring);

    // Let's finish by finding the offset in the ring (input - end of previous ring).
    let offset = input - ((ring - 1) * ring * 4 + 1);

    // And then relative to the side.
    let offset = (offset % (ring * 2) - ring).abs();
    println!("Input is {} steps from the ring diagonals", offset);

    println!("Total distance: {}", ring + offset);
}
