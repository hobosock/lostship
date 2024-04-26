pub enum PilotStatus {
    Normal,
    Injured,
    KIA,
}

pub enum Rank {
    Rookie,
    Veteran,
    Ace,
}

pub struct Pilot {
    name: String,
    kills: u64,
    rank: Rank,
    status: PilotStatus,
    injury_timer: u64,
}
