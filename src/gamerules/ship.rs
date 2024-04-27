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
    status: Status,
    upgrade: bool,
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
    name: String,
    damage: ShipDamage,
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
    position: u64,
    ship: Ship,
    pilot: Pilot,
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
