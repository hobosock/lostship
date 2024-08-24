use crate::gamerules::roll;

/// list of scout names for auto-generation
const SCOUT_LIST: [&str; 59] = [
    "UNSC Infinity",
    "Heighliner",
    "Tardis",
    "Battlestar Galactica",
    "Millenium Falcon",
    "Serenity",
    "Pathfinder",
    "Irregular Apocalypse",
    "No More Mr. Nice Guy",
    "Profit Margin",
    "Prosthetic Conscience",
    "Trade Surplus",
    "The Hand of God",
    "Cargo Cult",
    "So Much For Subtlety",
    "Zealot",
    "Gunboat Diplomat",
    "Last Arguments of Kings",
    "Screw Loose",
    "Bad For Business",
    "Only Slightly Bent",
    "Just Testing",
    "Xenophobe",
    "Uninvited Guest",
    "Violence Is The Answer",
    "War Crime",
    "Problem Child",
    "Attitude Adjuster",
    "Frightspear",
    "Lasting Damage",
    "Vulgarian",
    "Winter Storm",
    "Scar Glamour",
    "Learned Response",
    "Arcadia",
    "Argonaut",
    "Aurora",
    "Dark Star",
    "Endurance",
    "Hyperion",
    "Hail Mary",
    "Jupiter 2",
    "Liberator",
    "Nautilus",
    "Nemesis",
    "Orion",
    "Shangri-La",
    "USG Ishimura",
    "Valkyrie",
    "USS Voyager",
    "Silver Wings of Morning",
    "Yamato",
    "Yggdrasil",
    "Thunderbolt",
    "Viper",
    "Raptor",
    "Endeavour",
    "Razor Crest",
    "Voot Runner",
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
