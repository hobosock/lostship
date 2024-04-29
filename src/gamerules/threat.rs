use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Threats {
    None,
    Mk1,
    Mk2,
    Mk3,
}

impl fmt::Display for Threats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            Threats::None => "None",
            Threats::Mk1 => "MK1",
            Threats::Mk2 => "MK2",
            Threats::Mk3 => "MK3",
        };
        write!(f, "{}", printable)
    }
}

/// enemy fighter stats
pub struct Fighter {
    pub model: Threats,
    pub hp: i64,
    pub guns: u64,
    pub fuel: u64,
}

impl Fighter {
    /// returns Mk1 Fighter
    pub fn mk1() -> Fighter {
        Fighter {
            model: Threats::Mk1,
            hp: 2,
            guns: 1,
            fuel: 3,
        }
    }
    /// returns Mk2 Fighter
    pub fn mk2() -> Fighter {
        Fighter {
            model: Threats::Mk2,
            hp: 5,
            guns: 2,
            fuel: 4,
        }
    }
    /// returns Mk3 Fighter
    pub fn mk3() -> Fighter {
        Fighter {
            model: Threats::Mk3,
            hp: 8,
            guns: 4,
            fuel: 5,
        }
    }
}
