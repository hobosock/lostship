use super::{
    pilot::{PilotStatus, Rank},
    roll,
    ship::{Scout, ShipDamage},
    threat::{Fighter, Threats},
};

/// tracks all the information for combat phase
/// formation fields are predefined arrays because it is not possible to field more than 6 ships
/// scout_half field tracks if scouts are going or if enemy is taking the turn, scouts go first
/// scout_turns/enemy_turns tracks when each ship takes it's turn
#[derive(Debug, Clone)]
pub struct Combat {
    pub rounds: u64,
    pub scout_formation: Vec<Scout>,
    pub enemy_formation: Vec<Threats>,
    pub enemy_stats: Vec<Fighter>,
    pub scout_turns: Vec<bool>,
    pub enemy_turns: Vec<bool>,
    pub scout_half: bool,
}

/// logic for scout attack - modifies roll based on pilot rank and returns damage
pub fn scout_attack(scout: &Scout) -> u64 {
    let mut modifier: i64 = match scout.ship.damage {
        ShipDamage::Normal => 0,
        ShipDamage::Half => -1,
        _ => 0,
    };
    match scout.pilot.rank {
        Rank::Rookie => {}
        Rank::Veteran => modifier += 1,
        Rank::Ace => modifier += 2,
    };
    let attack_result = roll(6) + modifier;
    if attack_result == 5 {
        1
    } else if attack_result == 6 && scout.pilot.status == PilotStatus::Injured {
        1
    } else if attack_result == 6 && scout.pilot.status == PilotStatus::Normal {
        2
    } else {
        0
    }
}
