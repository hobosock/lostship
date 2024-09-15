use std::cmp::min;

use crate::gamerules::ship::Scout;

use crate::{
    app::App,
    gamerules::{
        combat::{scout_attack, subsystem_damage},
        pilot::{PilotStatus, Rank},
        roll,
        ship::{ShipDamage, Status},
    },
};

pub fn anomoies(app: &mut App) {
    let result = roll(6) + roll(6);
    if result == 2 {
        let mut pilot_roll = roll(app.pilots.len() as i64) as usize;
        let pilot = app.pilots[pilot_roll].clone();
        app.pilots[pilot_roll].status = PilotStatus::Kia;
        if app.scouts.len() < pilot_roll {
            pilot_roll = app.scouts.len();
        }
        app.scouts[pilot_roll].ship.damage = ShipDamage::Destroyed;
        // quick and dirty combat simulation
        let defense_num = min(app.scouts.len(), app.pilots.len());
        let mut attack_num = roll(6) - defense_num as i64;
        if app.mining_laser.status != Status::Inoperable {
            attack_num -= 1;
        }
        if attack_num < 1 {
            attack_num = 1;
        }
        let mut scout = app.scouts[pilot_roll].clone();
        scout.pilot = pilot;
        for _i in 0..attack_num {
            let attack_result = scout_attack(&scout);
            if attack_result > 4 {
                let target = roll(6);
                if target == 1 {
                    app.hull_damage += 1;
                } else if target == 2 {
                    app.engine.status = subsystem_damage(&app.engine.status);
                } else if target == 3 {
                    app.mining_laser.status = subsystem_damage(&app.mining_laser.status);
                } else if target == 4 {
                    app.scout_bay.status = subsystem_damage(&app.scout_bay.status);
                } else if target == 5 {
                    app.sick_bay.status = subsystem_damage(&app.sick_bay.status);
                } else {
                    app.sensors.status = subsystem_damage(&app.sensors.status);
                }
            }
        }
    } else if result == 3 {
        let asteroid_damage = (roll(6) + 1) / 2;
        app.game_text += &format!("\nAsteroid field!  Ship takes {} damage", asteroid_damage);
        app.hull_damage += asteroid_damage as u64;
    } else if result == 4 {
        app.game_text += "\nGravity well!  Leaving this system costs two fuel.";
        app.fuel -= 1;
    } else if result == 5 {
        app.game_text += "\nAirlock mishap!  Lost 3 parts.";
        if app.parts <= 3 {
            app.parts = 0;
        } else {
            app.parts -= 3;
        }
    } else if result == 6 {
        let mut pilot_roll = roll(6) as usize;
        if app.pilots.len() < pilot_roll {
            pilot_roll = app.pilots.len();
        }
        app.game_text += &format!(
            "\n{} is selected for an scouting mission and never returns.",
            app.pilots[pilot_roll].name
        );
        app.pilots[pilot_roll].status = PilotStatus::Kia;
    } else if result == 7 {
        let ss_roll = roll(6) + 6;
        let ss_name = if ss_roll == 7 {
            app.hull_damage += 2;
            "hull"
        } else if ss_roll == 8 {
            app.engine.status = subsystem_damage(&app.engine.status);
            app.engine.status = subsystem_damage(&app.engine.status);
            "engine"
        } else if ss_roll == 9 {
            app.mining_laser.status = subsystem_damage(&app.mining_laser.status);
            app.mining_laser.status = subsystem_damage(&app.mining_laser.status);
            "mining laser"
        } else if ss_roll == 10 {
            app.scout_bay.status = subsystem_damage(&app.scout_bay.status);
            app.scout_bay.status = subsystem_damage(&app.scout_bay.status);
            "scout bay"
        } else if ss_roll == 11 {
            app.sick_bay.status = subsystem_damage(&app.sick_bay.status);
            app.sick_bay.status = subsystem_damage(&app.sick_bay.status);
            "sick bay"
        } else {
            app.sensors.status = subsystem_damage(&app.sensors.status);
            app.sensors.status = subsystem_damage(&app.sensors.status);
            "sensors"
        };
        app.game_text += "\nIon storm!  The ";
        app.game_text += ss_name;
        app.game_text += " takes 2 damage.";
    } else if result == 8 {
        let mut pilot_roll = roll(6) as usize;
        if app.pilots.len() < pilot_roll {
            pilot_roll = app.pilots.len();
        }
        app.game_text += &format!(
            "{} is selected for an scouting mission and returns with more experience.",
            app.pilots[pilot_roll].name
        );
        app.pilots[pilot_roll].rank = match app.pilots[pilot_roll].rank {
            Rank::Rookie => Rank::Veteran,
            Rank::Veteran => Rank::Ace,
            Rank::Ace => Rank::Ace,
        };
    } else if result == 9 {
        app.game_text += "\nShip graveyard!  Found 3 parts.";
        app.parts += 3;
    } else if result == 10 {
        app.game_text += "\nDiscoverd some kind of alien technology!  The ";
        let ss_roll = roll(6) + 6;
        let ss_name = if ss_roll == 7 {
            app.hull_damage = 0;
            app.hull_upgrade = true;
            "hull"
        } else if ss_roll == 8 {
            app.engine.status = Status::Normal;
            app.engine.upgrade = true;
            "engine"
        } else if ss_roll == 9 {
            app.mining_laser.status = Status::Normal;
            app.mining_laser.upgrade = true;
            "mining laser"
        } else if ss_roll == 10 {
            app.scout_bay.status = Status::Normal;
            app.scout_bay.upgrade = true;
            "scout bay"
        } else if ss_roll == 11 {
            app.sick_bay.status = Status::Normal;
            app.sick_bay.upgrade = true;
            "sick bay"
        } else {
            app.sensors.status = Status::Normal;
            app.sensors.upgrade = true;
            "sensors"
        };
        app.game_text += ss_name;
        app.game_text += " is fully repaired and upgraded.";
    } else if result == 11 {
        app.game_text += "\nEncountered a healing field!  Hull fully repaired.";
        app.hull_damage = 0;
    } else {
        if !app.honor_roll.is_empty() {
            app.pilots.push(app.honor_roll.pop().unwrap()); // unwrap is ok?
            if app.scouts.len() < 6 {
                let mut new_scout = Scout::default();
                new_scout.ship.new_name(&mut app.scouts_in_use);
                app.scouts.push(new_scout);
            }
        }
    }
}
