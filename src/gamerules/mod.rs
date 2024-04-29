pub mod game_functions;
pub mod pilot;
pub mod ship;
pub mod threat;

use rand::Rng;

#[derive(Debug, Clone)]
pub enum ScanResult {
    Barren,
    Fuel,
    Anomoly,
    Home,
}

#[derive(Debug, Default)]
pub struct Leap {
    combat_rounds: u64,
    parts_found: u64,
    fuel_found: u64,
}

pub fn roll(side: i64) -> i64 {
    if side == 1 {
        1
    } else {
        rand::thread_rng().gen_range(1..side)
    }
}
