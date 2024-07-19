use crate::gamerules::roll;

/// list of pilot names for auto-generation
const NAME_LIST: [&str; 6] = [
    "Seth",
    "Mad Max",
    "Fernando Alonso",
    "Malcom Reynolds",
    "Bubba",
    "Bartimaeus",
];

/// produces a random name from "NAME_LIST"
/// avoids duplicates by tracking previous rolls
pub fn generate_pilot_name(in_use: &mut Option<Vec<usize>>) -> String {
    let length = NAME_LIST.len() as i64;
    let mut idx = (roll(length) - 1) as usize;
    let mut duplicate = true;
    if in_use.is_some() {
        while duplicate {
            match in_use.as_ref().unwrap().iter().find(|x| **x == idx) {
                Some(_) => idx = roll(length) as usize,
                None => duplicate = false,
            }
        }
        let mut temp = in_use.clone().unwrap();
        temp.push(idx);
        *in_use = Some(temp);
    } else {
        *in_use = Some(vec![idx]);
    }
    NAME_LIST[idx].to_string()
}
