use core::fmt;

use super::pilot::Pilot;

/// SubSystem status (100/66/33/0%)
#[derive(Debug)]
pub enum Status {
    Normal,
    Serviceable,
    BarelyFunctioning,
    Inoperable,
}

impl Default for Status {
    fn default() -> Self {
        Status::Normal
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            Status::Normal => "Normal",
            Status::Serviceable => "Serviceable",
            Status::BarelyFunctioning => "Barely Functioning",
            Status::Inoperable => "Inoperable",
        };
        write!(f, "{}", printable)
    }
}

/// Scout damage
#[derive(Debug, Clone)]
pub enum ShipDamage {
    Normal,
    Half,
    Inoperable,
    Destroyed,
}

impl Default for ShipDamage {
    fn default() -> Self {
        ShipDamage::Normal
    }
}

/// represents each subsystem, tracks status and upgrade
#[derive(Debug)]
pub struct SubSystem {
    pub status: Status,
    pub upgrade: bool,
}

impl Default for SubSystem {
    fn default() -> Self {
        SubSystem {
            status: Status::default(),
            upgrade: false,
        }
    }
}

/// represents each scout ship, tracks damage and name
#[derive(Debug, Clone)]
pub struct Ship {
    pub name: String,
    pub damage: ShipDamage,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            name: "Ship Name".to_string(),
            damage: ShipDamage::default(),
        }
    }
}

/// Scout struct with Pilot, Ship, and Position
#[derive(Debug, Clone)]
pub struct Scout {
    pub position: u64,
    pub ship: Ship,
    pub pilot: Pilot,
}

impl Default for Scout {
    fn default() -> Self {
        Scout {
            position: 0,
            ship: Ship::default(),
            pilot: Pilot::default(),
        }
    }
}
