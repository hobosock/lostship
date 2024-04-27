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

#[derive(Debug, Clone)]
pub enum Rank {
    Rookie,
    Veteran,
    Ace,
}

impl Default for Rank {
    fn default() -> Self {
        Rank::Rookie
    }
}

#[derive(Debug, Clone)]
pub struct Pilot {
    name: String,
    kills: u64,
    rank: Rank,
    status: PilotStatus,
    injury_timer: u64,
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
