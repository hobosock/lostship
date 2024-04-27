pub mod pilot;
pub mod ship;

use rand::Rng;

#[derive(Debug)]
pub struct Leap {
    combat_rounds: u64,
    parts_found: u64,
    fuel_found: u64,
}

impl Default for Leap {
    fn default() -> Self {
        Leap {
            combat_rounds: 0,
            parts_found: 0,
            fuel_found: 0,
        }
    }
}

pub fn roll(side: u64) -> u64 {
    if side == 1 {
        1
    } else {
        rand::thread_rng().gen_range(1..side)
    }
}
