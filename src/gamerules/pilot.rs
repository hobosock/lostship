use core::fmt;

#[derive(Debug, Clone)]
pub enum PilotStatus {
    Normal,
    Injured,
    KIA,
}

impl Default for PilotStatus {
    fn default() -> Self {
        PilotStatus::Normal
    }
}

impl fmt::Display for PilotStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            PilotStatus::Normal => "Normal",
            PilotStatus::Injured => "Injured",
            PilotStatus::KIA => "KIA",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug, Clone)]
pub enum Rank {
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
        write!(f, "{}", printable)
    }
}

impl Default for Rank {
    fn default() -> Self {
        Rank::Rookie
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
            name: "Pilot".to_string(),
            kills: 0,
            rank: Rank::default(),
            status: PilotStatus::default(),
            injury_timer: 0,
        }
    }
}
