/// Simple Thanos Sort
/// Author: Dock O'Neal

extern crate rand;

use std::env;
use rand::distributions::{Distribution, Uniform};

fn sorted(universe: &Vec<i32>) -> bool {
    for i in 0..universe.len()-1 {
        if universe[i] > universe[i+1] {
            return false
        }
    }
    true
}

/// Snap to eliminate half the universe at random
fn snap(universe: &mut Vec<i32>) -> Vec<i32> {
    let mid = universe.len() / 2;
    let mut rng = rand::thread_rng();
    let mut soul_stone = vec![];
    while universe.len() > mid {
        let snap_range = Uniform::new_inclusive(0, universe.len() - 1);
        soul_stone.push(universe.remove(snap_range.sample(&mut rng)));
    }
    soul_stone
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // First argument is the executable, remove it
    args.remove(0);
    let mut universe: Vec<i32> = if args.len() > 0 {
        // Extract user input as a vector of 32 bit floats
        args
            .iter()
            .map(|arg| arg.parse::<i32>().unwrap())
            .collect()
    } else {
        // User has not given us a universe, generate it
        let mut rng = rand::thread_rng();
        let range = Uniform::new_inclusive(0, 100);
        let mut v = vec![];
        for _i in 0..50 {
            v.push(range.sample(&mut rng) as i32);
        }
        v
    };

    println!("Thanos is sorting: {:?}\n", universe);
    let mut soul_stone: Vec<i32> = vec![];
    while !sorted(&universe) {
       println!("The universe is not sorted!");
       println!("Thanos snaps!");
       soul_stone.append(&mut snap(&mut universe));
       println!("The universe is now: {:?}\n", universe);
    }
    println!("The universe is sorted");
    println!("Thanos puts on his farmer hat");
}
