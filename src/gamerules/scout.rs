use crate::app::App;

use super::ship::ShipDamage;

pub fn scout_repair(app: &mut App, position: usize) {
    match app.scouts[position].ship.damage {
        ShipDamage::Normal => app.scouts[position].ship.damage = ShipDamage::Normal,
        ShipDamage::Half => app.scouts[position].ship.damage = ShipDamage::Normal,
        ShipDamage::Inoperable => {
            if app.parts >= 1 {
                app.parts -= 1;
                app.scouts[position].ship.damage = ShipDamage::Normal;
            }
        }
        ShipDamage::Destroyed => {
            if app.parts >= 6 {
                app.parts -= 6;
                app.scouts[position].ship.damage = ShipDamage::Normal;
            }
        }
    }
}
