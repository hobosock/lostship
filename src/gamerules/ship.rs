use core::fmt;

use crate::app::App;

use super::pilot::Pilot;

/// SubSystem status (100/66/33/0%)
#[derive(Debug, Default)]
pub enum Status {
    #[default]
    Normal,
    Serviceable,
    BarelyFunctioning,
    Inoperable,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            Status::Normal => "Normal",
            Status::Serviceable => "Serviceable",
            Status::BarelyFunctioning => "Barely Functioning",
            Status::Inoperable => "Inoperable",
        };
        write!(f, "{printable}")
    }
}

/// Scout damage
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ShipDamage {
    #[default]
    Normal,
    Half,
    Inoperable,
    Destroyed,
}

impl fmt::Display for ShipDamage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            ShipDamage::Normal => "Normal",
            ShipDamage::Half => "Half",
            ShipDamage::Inoperable => "Inoperable",
            ShipDamage::Destroyed => "Destroyed",
        };
        write!(f, "{}", printable)
    }
}

/// represents each subsystem, tracks status and upgrade
#[derive(Debug, Default)]
pub struct SubSystem {
    pub status: Status,
    pub upgrade: bool,
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
#[derive(Debug, Clone, Default)]
pub struct Scout {
    pub position: u64,
    pub ship: Ship,
    pub pilot: Pilot,
}

/// repairs selected subsystem by one level
pub fn subsystem_repair(app: &mut App, subsystem: usize) {
    if app.parts >= 2 && subsystem > 0 {
        // TODO: check if subsystem is actually damaged first
        app.parts -= 2;
        app.game_text = "Subsystem fully repaired with 2 parts.".to_string();
        if subsystem == 0 {
            //
        }
    } else if app.parts >= 1 && subsystem == 0 && app.hull_damage > 0 {
        app.parts -= 1;
        app.hull_damage -= 1;
        app.game_text = "Hull damage repaired with 1 part.".to_string();
    } else {
        app.game_text = "Not enough parts to make this repair!".to_string();
    }
}
