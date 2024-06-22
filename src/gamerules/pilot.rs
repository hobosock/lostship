use core::fmt;

use super::threat::Threats;

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
    pub fn mark_kill(&mut self, enemy: &Threats) {
        match enemy {
            Threats::Mk1 => self.kills += 1,
            Threats::Mk2 => self.kills += 2,
            Threats::Mk3 => self.kills += 3,
            Threats::None => {}
        }
    }
    pub fn rank_up(&mut self) {
        if self.kills >= 6 {
            self.rank = Rank::Ace;
        } else if self.kills >= 3 && self.kills < 6 {
            self.rank = Rank::Veteran;
        }
    }
}
