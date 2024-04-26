/// SubSystem status (100/66/33/0%)
pub enum Status {
    Normal,
    Serviceable,
    BarelyFunctioning,
    Inoperable,
}

/// Scout damage
pub enum ShipDamage {
    Normal,
    Half,
    Inoperable,
    Destroyed,
}

/// represents each subsystem, tracks status and upgrade
pub struct SubSystem {
    status: Status,
    upgrade: bool,
}

/// represents each scout ship, tracks damage and name
pub struct Ship {
    name: String,
    damage: ShipDamage,
}
