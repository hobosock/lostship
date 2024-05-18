pub mod combat;
pub mod game_functions;
pub mod pilot;
pub mod scout;
pub mod ship;
pub mod threat;

use core::fmt;

use rand::Rng;

#[derive(Debug, Clone)]
pub enum ScanResult {
    Barren,
    Fuel,
    Anomoly,
    Home,
}

impl fmt::Display for ScanResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            ScanResult::Barren => "Barren",
            ScanResult::Fuel => "Fuel",
            ScanResult::Anomoly => "Anomoly",
            ScanResult::Home => "Home",
        };
        write!(f, "{}", printable)
    }
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
