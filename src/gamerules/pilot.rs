use core::fmt;

use crate::app::{self, App};

use super::{
    ship::{Status, SubSystem},
    threat::Threats,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum PilotStatus {
    #[default]
    Normal,
    Injured,
    Kia,
}

impl fmt::Display for PilotStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            PilotStatus::Normal => "Normal",
            PilotStatus::Injured => "Injured",
            PilotStatus::Kia => "KIA",
        };
        write!(f, "{printable}")
    }
}

#[derive(Debug, Clone, Default)]
pub enum Rank {
    #[default]
    Rookie,
    Veteran,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            Rank::Rookie => "Rookie",
            Rank::Veteran => "Veteran",
            Rank::Ace => "Ace",
        };
        write!(f, "{printable}")
    }
}

#[derive(Debug, Clone)]
pub struct Pilot {
    pub name: String,
    pub kills: u64,
    pub rank: Rank,
    pub status: PilotStatus,
    pub injury_timer: u64,
}

impl Default for Pilot {
    fn default() -> Self {
        Pilot {
            name: "Pilot".to_string(), // TODO: replace with name generation function
            kills: 0,
            rank: Rank::default(),
            status: PilotStatus::default(),
            injury_timer: 0,
        }
    }
}

impl Pilot {
    /// increases pilot kill count based on enemy ship type
    pub fn mark_kill(&mut self, enemy: &Threats) {
        match enemy {
            Threats::Mk1 => self.kills += 1,
            Threats::Mk2 => self.kills += 2,
            Threats::Mk3 => self.kills += 3,
            Threats::None => {}
        }
    }
    /// increases pilot rank if it has the requisite number of kills
    pub fn rank_up(&mut self) {
        if self.kills >= 6 {
            self.rank = Rank::Ace;
        } else if self.kills >= 3 && self.kills < 6 {
            self.rank = Rank::Veteran;
        }
    }
    /// improves pilot's status based on Sick Bay condition
    /// ticks up injury timer until healed, then resets to 0
    pub fn heal(&mut self, sick_bay: &SubSystem) {
        // if sick bay is upgraded, injured pilots are healed at the end of battle
        // 100% - 1 leap
        // 66% - 2 leaps
        // 33% - 3 leaps
        // Inoperable - newly injured pilots die
        if self.status == PilotStatus::Injured {
            match sick_bay.status {
                Status::Normal => {
                    if sick_bay.upgrade {
                        self.status = PilotStatus::Normal;
                        self.injury_timer = 0;
                        return;
                    } else if self.injury_timer > 0 {
                        self.status = PilotStatus::Normal;
                        self.injury_timer = 0;
                        return;
                    }
                }
                Status::Serviceable => {
                    if self.injury_timer > 1 {
                        self.status = PilotStatus::Normal;
                        self.injury_timer = 0;
                        return;
                    }
                }
                Status::BarelyFunctioning => {
                    if self.injury_timer > 2 {
                        self.status = PilotStatus::Normal;
                        self.injury_timer = 0;
                        return;
                    }
                }
                Status::Inoperable => {
                    self.status = PilotStatus::Kia;
                    return;
                }
            }
            self.injury_timer += 1; // tick up if not healed
        }
    }
}

/// create list item of Pilots in honor roll
/// text has pilot name and number of kills
pub fn honor_roll_to_list(roll: &[Pilot]) -> Vec<String> {
    let mut honor_list: Vec<String> = Vec::new();
    for pilot in roll.iter() {
        honor_list.push(format!("{} {}", pilot.name, pilot.kills));
    }
    honor_list
}

// TODO: use this in other places like fn a_key_press()
/// copies pilot info from app.pilots into scout pilot information and scout formation pilot
/// information
/// there is probably a better way to do this, just hacking it together for now
pub fn update_pilot_info(app: &mut App) {
    for (i, pilot) in app.pilots.iter().enumerate() {
        app.scouts[i].pilot = pilot.clone();
    }
}
