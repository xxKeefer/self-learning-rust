// Given two crystal balls that will break if dropped from high enough
// distance, determine the exact spot in which it will break in the most
// optimized way.
use std::cmp::min;

#[derive(Debug)]
struct Tower {
    floors: Vec<bool>,
}

impl Tower {
    fn new(height: usize) -> Self {
        let mut build: Vec<bool> = (0..=height).map(|_| rand::random::<bool>()).collect();
        // force a false state
        // let mut build: Vec<bool> = (0..=height).map(|_| false).collect();
        build.sort();
        Self { floors: build }
    }

    fn breaks(&self) -> Option<usize> {
        let mut first_broken = false;
        let mut lo: usize = 0;
        let step: usize = (self.floors.len() as f64).sqrt().floor() as usize;
        let mut hi: usize = step;

        while !first_broken && hi < self.floors.len() {
            if self.floors[hi] {
                first_broken = true;
            } else {
                lo = hi;

                hi = min(hi + step, self.floors.len());
            }
        }

        while lo < hi {
            if self.floors[lo] {
                return Some(lo);
            };
            lo += 1;
        }
        None
    }
}

fn main() {
    let tower = Tower::new(10);
    let breaks_at = tower.breaks();

    match breaks_at {
        Some(index) => println!(
            "<-FALSE| {} < {} > {} |TRUE->",
            tower.floors[index - 1],
            tower.floors[index],
            tower.floors[index + 1]
        ),
        None => println!("Balls did not break for this tower."),
    }
}
