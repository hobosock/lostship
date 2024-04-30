/* Order of Play
 * 1. leap into system
 * 2. assess threat
 * 3. fight (if necessary)
 * 4. search wreckage for parts
 * 5. scan the system
 * 6. make repairs and bury your dead
 * 7. leap again
 */

use std::ops::RangeBounds;

use crate::app::App;

use super::{roll, threat::Threats, ScanResult};

/// enum for tracking which step of the jump the player is in
#[derive(Debug)]
pub enum JumpStep {
    Step1,
    Step2,
    Step3,
    Step4,
    Step5,
    Step6,
    Step7,
}

/// Step 1. leap into system
pub fn leap_into_system(app: &mut App) {
    app.leaps_since_incident += 1;
    if app.fuel > 0 {
        app.fuel -= 1;
    } else {
        app.game_text += "Out of fuel!  Game Over";
    }
}

/// Step 2. assess threat
pub fn assess_threat(app: &App) -> Option<Vec<Threats>> {
    let roll_mod: i64 = if app.leaps_since_incident == 1 {
        -3
    } else if app.leaps_since_incident == 2 {
        -2
    } else if app.leaps_since_incident == 3 {
        -1
    } else if app.leaps_since_incident >= 4 && app.leaps_since_incident <= 7 {
        0
    } else {
        app.leaps_since_incident as i64 - 7
    };

    let threat_result = roll(6) + roll(6) + roll_mod;

    if threat_result <= 3 {
        None
    } else if threat_result == 4 {
        Some(vec![Threats::Mk1; 4])
    } else if threat_result == 5 {
        Some(vec![Threats::Mk1; 5])
    } else if threat_result == 6 {
        Some(vec![Threats::Mk1; 6])
    } else if threat_result == 7 {
        Some(vec![Threats::Mk2])
    } else if threat_result == 8 {
        Some(vec![Threats::Mk2, Threats::Mk1, Threats::Mk1])
    } else if threat_result == 9 {
        let mut threat = vec![Threats::Mk2];
        threat.append(&mut vec![Threats::Mk1; 3]);
        Some(threat)
    } else if threat_result == 10 {
        Some(vec![Threats::Mk2; 2])
    } else if threat_result == 11 {
        Some(vec![Threats::Mk2, Threats::Mk2, Threats::Mk1])
    } else if threat_result == 12 {
        Some(vec![Threats::Mk3])
    } else if threat_result == 13 {
        let mut threat = vec![Threats::Mk3];
        threat.append(&mut vec![Threats::Mk1; 3]);
        Some(threat)
    } else if threat_result == 14 {
        Some(vec![Threats::Mk3, Threats::Mk2])
    } else {
        let mut threat = vec![Threats::Mk3; 2];
        threat.append(&mut vec![Threats::Mk2; 2]);
        threat.append(&mut vec![Threats::Mk1; 2]);
        Some(threat)
    }
}

/// Step 4. search wreckage for parts
pub fn search_wreckage(threats: Vec<Threats>) -> u64 {
    let mut bmk2 = false;
    let mut bmk3 = false;
    for threat in threats.iter() {
        if *threat == Threats::Mk2 {
            bmk2 = true;
        } else if *threat == Threats::Mk3 {
            bmk3 = true;
        }
    }
    let roll_mod = if bmk3 && bmk2 {
        3
    } else if bmk3 {
        2
    } else if bmk2 {
        1
    } else {
        0
    };
    roll(6) as u64 + roll_mod
}

/// Step 5. scan the system
pub fn system_scan(leaps: u64) -> (u64, ScanResult) {
    let roll_mod = if leaps == 1 {
        -3
    } else if leaps == 2 {
        -2
    } else if leaps == 3 {
        -1
    } else if (4..=7).contains(&leaps) {
        0
    } else {
        1
    };

    let system_scan = roll(6) + roll(6) + roll_mod;
    if system_scan < 6 {
        (0, ScanResult::Barren)
    } else if system_scan == 6 || system_scan == 8 {
        (1, ScanResult::Fuel)
    } else if system_scan == 7 {
        // TODO: anomaly
        (0, ScanResult::Anomoly)
    } else if system_scan == 9 {
        (2, ScanResult::Fuel)
    } else if system_scan == 10 {
        (3, ScanResult::Fuel)
    } else {
        (0, ScanResult::Home)
    }
}
