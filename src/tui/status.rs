use ratatui::prelude::Span;
use ratatui::style::Stylize;

use crate::gamerules::ship::{Status, SubSystem};

/// returns colored string of fuel amount
/// green -> yellow -> red as fuel gets lower
pub fn get_fuel_string(fuel: &u64) -> Span {
    if *fuel > 4 {
        fuel.to_string().green()
    } else if *fuel <= 4 && *fuel > 2 {
        fuel.to_string().yellow()
    } else {
        fuel.to_string().red()
    }
}

/// returns colored string of parts amount
/// green -> yellow -> red as fuel gets lower
pub fn get_parts_string(parts: &u64) -> Span {
    if *parts >= 6 {
        parts.to_string().green()
    } else if *parts < 6 && *parts >= 3 {
        parts.to_string().yellow()
    } else {
        parts.to_string().red()
    }
}

/// returns colored string of hull damage
/// green -> yellow -> red as damge increases
pub fn get_hull_string(damage: u64, upgraded: bool) -> Span<'static> {
    let max: u64 = if upgraded { 7 } else { 6 };
    if damage > 4 {
        format!("{} / {}", damage, max).red()
    } else if damage <= 4 && damage > 2 {
        format!("{} / {}", damage, max).yellow()
    } else {
        format!("{} / {}", damage, max).green()
    }
}

/// returns colored string of given subsystem status
pub fn get_subsys_string(subsystem: &SubSystem) -> Span<'static> {
    match subsystem.status {
        Status::Normal => "Normal".green(),
        Status::Serviceable => "Serviceable".magenta(),
        Status::BarelyFunctioning => "Barely Functioning".yellow(),
        Status::Inoperable => "Inoperable".red(),
    }
}
