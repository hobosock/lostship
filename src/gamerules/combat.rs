use crate::app::App;

use super::{
    pilot::{PilotStatus, Rank},
    roll,
    ship::{Scout, ShipDamage, Status},
    threat::{Fighter, Threats},
};

pub enum Targets {
    Superficial,
    FifthScout,
    FourthScout,
    ThirdScout,
    SecondScout,
    LeadScout,
    Hull,
    Engines,
    MiningLaser,
    ScoutingBay,
    SickBay,
    Sensors,
}

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
    pub laser_fired: bool, // has mining laser been fired this turn?
    pub combat_text: String,
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
    if attack_result == 5 || (attack_result == 6 && scout.pilot.status == PilotStatus::Injured) {
        1
    } else if attack_result == 6 && scout.pilot.status == PilotStatus::Normal {
        2
    } else {
        0
    }
}

/// enemy attack success roll
pub fn enemy_attack() -> bool {
    roll(6) > 3
}

/// logic for enemy targeting - handles 1st round, 2nd round and after
pub fn enemy_targeting(combat: &Combat) -> Targets {
    let roll_result = if combat.rounds > 1 {
        roll(6) + roll(6)
    } else {
        roll(6)
    };
    if roll_result == 1 {
        Targets::Superficial
    } else if roll_result == 2 {
        Targets::FifthScout
    } else if roll_result == 3 {
        Targets::FourthScout
    } else if roll_result == 4 {
        Targets::ThirdScout
    } else if roll_result == 5 {
        Targets::SecondScout
    } else if roll_result == 6 {
        Targets::LeadScout
    } else if roll_result == 7 {
        Targets::Hull
    } else if roll_result == 8 {
        Targets::Engines
    } else if roll_result == 9 {
        Targets::MiningLaser
    } else if roll_result == 10 {
        Targets::ScoutingBay
    } else if roll_result == 11 {
        Targets::SickBay
    } else if roll_result == 12 {
        Targets::Sensors
    } else {
        Targets::Hull
    }
}

/// logic for damaging scout on hit
pub fn scout_damage(scout: &mut Scout) -> String {
    let roll_result = roll(6);
    if roll_result == 1 {
        "Superficial damage.".to_string()
    } else if roll_result == 2 {
        match scout.pilot.status {
            PilotStatus::Normal => {
                scout.pilot.status = PilotStatus::Injured;
                "Pilot injured".to_string()
            }
            PilotStatus::Injured => {
                scout.pilot.status = PilotStatus::Kia;
                "Injured pilot KIA".to_string()
            }
            _ => {
                "...".to_string() // NOTE: shouldn't land here
            }
        }
    } else if roll_result == 3 {
        scout.pilot.status = PilotStatus::Kia;
        "Pilot KIA".to_string()
    } else if roll_result == 4 {
        match scout.ship.damage {
            ShipDamage::Normal => {
                scout.ship.damage = ShipDamage::Half;
                "Scout at 50% damage".to_string()
            }
            ShipDamage::Half => {
                scout.ship.damage = ShipDamage::Destroyed;
                "Damaged scout is destroyed".to_string()
            }
            _ => "...".to_string(), //NOTE: shouldn't land here
        }
    } else if roll_result == 5 {
        scout.ship.damage = ShipDamage::Inoperable;
        "Scout Inoperable, recalling now...".to_string()
    } else {
        scout.pilot.status = PilotStatus::Kia;
        scout.ship.damage = ShipDamage::Destroyed;
        "Scout destroyed, pilot KIA".to_string()
    }
}

/// logic for mining laser attack
pub fn mining_laser(upgraded: bool) -> u64 {
    let mut roll_result = roll(6);
    if upgraded {
        roll_result += 1;
    }
    if (4..=5).contains(&roll_result) {
        1
    } else if roll_result == 6 {
        2
    } else if roll_result == 7 {
        3
    } else {
        0
    }
}

/// handles subtraction for enemy damage, protects for overflow
pub fn enemy_damage(damage: u64, hp: u64) -> u64 {
    if hp < damage {
        0
    } else {
        hp - damage
    }
}

/// function for rendering enemy turns in combat
pub fn enemy_turn(combat: &mut Combat, app: &mut App) {
    for (i, turn) in combat.enemy_turns.clone().iter().enumerate() {
        if !turn {
            combat.enemy_turns[i] = true;
            if combat.enemy_stats[i].hp == 0 || combat.enemy_stats[i].fuel == 0 {
                continue; // stops from hanging on dead fighter
            }
            combat.combat_text = String::new();
            let guns = combat.enemy_stats[i].guns;
            for _ in 0..guns {
                if enemy_attack() {
                    let target = enemy_targeting(combat);
                    match target {
                        Targets::Superficial => {
                            combat.combat_text += &format!(
                                "Enemy {} deals superficial damage!  ",
                                combat.enemy_stats[i].model
                            );
                        }
                        // TODO: check for hull damage instead
                        Targets::FifthScout => {
                            let damage_text = scout_damage(&mut combat.scout_formation[4]);
                            combat.combat_text += &format!(
                                "Enemy {} damages {}.  Scout {}  ",
                                combat.enemy_stats[i].model,
                                combat.scout_formation[4].ship.name,
                                damage_text
                            );
                        }
                        Targets::FourthScout => {
                            let damage_text = scout_damage(&mut combat.scout_formation[3]);
                            combat.combat_text += &format!(
                                "Enemy {} damages {}.  Scout {}  ",
                                combat.enemy_stats[i].model,
                                combat.scout_formation[3].ship.name,
                                damage_text
                            );
                        }
                        Targets::ThirdScout => {
                            let damage_text = scout_damage(&mut combat.scout_formation[2]);
                            combat.combat_text += &format!(
                                "Enemy {} damages {}.  Scout {}  ",
                                combat.enemy_stats[i].model,
                                combat.scout_formation[2].ship.name,
                                damage_text
                            );
                        }
                        Targets::SecondScout => {
                            let damage_text = scout_damage(&mut combat.scout_formation[1]);
                            combat.combat_text += &format!(
                                "Enemy {} damages {}.  Scout {}  ",
                                combat.enemy_stats[i].model,
                                combat.scout_formation[1].ship.name,
                                damage_text
                            );
                        }
                        Targets::LeadScout => {
                            let damage_text = scout_damage(&mut combat.scout_formation[0]);
                            combat.combat_text += &format!(
                                "Enemy {} damages {}.  Scout {}  ",
                                combat.enemy_stats[i].model,
                                combat.scout_formation[0].ship.name,
                                damage_text
                            );
                        }
                        Targets::Hull => {
                            app.hull_damage += 1;
                            combat.combat_text += &format!(
                                "Enemy {} damages the hull.  ",
                                combat.enemy_stats[i].model,
                            );
                        }
                        Targets::Engines => {
                            app.engine.status = subsystem_damage(&app.engine.status);
                            combat.combat_text += &format!(
                                "Enemy {} damages the engines.  ",
                                combat.enemy_stats[i].model,
                            );
                        }
                        Targets::MiningLaser => {
                            app.mining_laser.status = subsystem_damage(&app.mining_laser.status);
                            combat.combat_text += &format!(
                                "Enemy {} damages the mining laser.  ",
                                combat.enemy_stats[i].model,
                            );
                        }
                        Targets::ScoutingBay => {
                            app.scout_bay.status = subsystem_damage(&app.scout_bay.status);
                            combat.combat_text += &format!(
                                "Enemy {} damages the scout bay.  ",
                                combat.enemy_stats[i].model,
                            );
                        }
                        Targets::SickBay => {
                            app.sick_bay.status = subsystem_damage(&app.sick_bay.status);
                            combat.combat_text += &format!(
                                "Enemy {} damages the sick bay.  ",
                                combat.enemy_stats[i].model,
                            );
                        }
                        Targets::Sensors => {
                            app.sensors.status = subsystem_damage(&app.sensors.status);
                            combat.combat_text += &format!(
                                "Enemy {} damages the sensors.  ",
                                combat.enemy_stats[i].model,
                            );
                        }
                    }
                } else {
                    combat.combat_text += "Miss!  ";
                }
            }
            break;
        }
    }
}

/// decrements sub system rating when sustaining damage
pub fn subsystem_damage(status: &Status) -> Status {
    match status {
        Status::Normal => Status::Serviceable,
        Status::Serviceable => Status::BarelyFunctioning,
        Status::BarelyFunctioning => Status::Inoperable,
        Status::Inoperable => Status::Inoperable,
    }
}

// NOTE: there is probably a better way to handle this, I'll worry about that later
/// copies scout changes from the combat struct back into the app struct
pub fn combat_to_app(combat: &Combat, app: &mut App) {
    for (i, scout) in combat.scout_formation.iter().enumerate() {
        app.scouts[i] = scout.clone();
        app.pilots[i] = scout.pilot.clone();
    }
}
