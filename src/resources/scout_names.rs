use crate::gamerules::roll;

/// list of scout names for auto-generation
const SCOUT_LIST: [&str; 7] = [
    "UNSC Infinity",
    "Heighliner",
    "Tardis",
    "Battlestar Galactica",
    "Millenium Falcon",
    "Serenity",
    "Pathfinder",
];

/// produces a random name from "SCOUT_LIST"
/// avoids duplicates by tracking previous rolls
pub fn generate_scout_name(in_use: &mut Option<Vec<usize>>) -> String {
    let length = SCOUT_LIST.len() as i64;
    let mut idx = (roll(length) - 1) as usize;
    let mut duplicate = true;
    if in_use.is_some() {
        while duplicate {
            match in_use.as_ref().unwrap().iter().find(|x| **x == idx) {
                Some(_) => idx = (roll(length) - 1) as usize,
                None => duplicate = false,
            }
        }
        let mut temp = in_use.clone().unwrap();
        temp.push(idx);
        *in_use = Some(temp);
    } else {
        *in_use = Some(vec![idx]);
    }
    SCOUT_LIST[idx].to_string()
}
