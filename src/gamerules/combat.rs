use super::{ship::Scout, threat::Threats};

/// tracks all the information for combat phase
/// formation fields are predefined arrays because it is not possible to field more than 6 ships
/// scout_half field tracks if scouts are going or if enemy is taking the turn, scouts go first
/// scout_turns/enemy_turns tracks when each ship takes it's turn
#[derive(Debug, Clone)]
pub struct Combat {
    pub rounds: u64,
    pub scout_formation: Vec<Scout>,
    pub enemy_formation: Vec<Threats>,
    pub scout_turns: Vec<bool>,
    pub enemy_turns: Vec<bool>,
    pub scout_half: bool,
}
