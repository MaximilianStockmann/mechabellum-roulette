use core::fmt::{self, Formatter};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UnitType {
    Air,
    Ground,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UnitAttackType {
    Ground,
    AirAndGround,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Unit {
    id: i32,
    name: String,
    unit_type: UnitType,
    unit_attack_type: UnitAttackType,
    is_giant: bool,
    is_rare: bool,
}

impl Unit {
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_attack_type(&self) -> &UnitAttackType {
        &self.unit_attack_type
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// TODO: Adjust so there is an equal distribution of T1, T2, T3 units
pub fn randomize_unit_types(unit_type_number: &i32, units: &Vec<Unit>) -> Vec<Unit> {
    let mut rng = thread_rng();
    let final_units;

    loop {
        let mut selected_units = Vec::<Unit>::new();
        let mut unit_id_cache: Vec<i32> = Vec::<i32>::new();

        // TODO: Calculate range based on units in unit list
        for _ in 0..*unit_type_number {
            let mut random_unit_id = rng.gen_range(1..=31);
            loop {
                if !unit_id_cache.contains(&random_unit_id) {
                    unit_id_cache.push(random_unit_id);
                    break;
                }
                random_unit_id = rng.gen_range(1..=27);
            }

            let random_unit = units.iter().find(|e| e.get_id() == random_unit_id).unwrap();
            selected_units.push(random_unit.clone());
        }

        if check_attack_types(&selected_units) {
            final_units = selected_units;
            break;
        }
    }

    final_units
}

fn check_attack_types(unit_list: &Vec<Unit>) -> bool {
    let mut all_attack_types_present = false;

    all_attack_types_present = unit_list
        .iter()
        .any(|unit| unit.get_attack_type() == &UnitAttackType::AirAndGround)
        || all_attack_types_present;

    all_attack_types_present
}

pub fn parse_unit_data() -> Result<Vec<Unit>, Box<dyn Error>> {
    let bytes = include_bytes!("../data/units.json");
    let s = std::str::from_utf8(bytes).unwrap();

    let units: Vec<Unit> = serde_json::from_str(&s)?;

    Ok(units)
}
