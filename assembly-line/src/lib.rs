// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    const PRODUCTIVITY: f64 = 221.00;

    let success_rate: f64 = match speed {
        0 => 0.00,
        1..=4 => 1.00,
        5..=8 => 0.90,
        9..=10 => 0.77,
        x => panic!("Unexpected speed  {:?}", x),
    };
    return speed as f64 * success_rate * PRODUCTIVITY;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) as u32 / 60);
}
