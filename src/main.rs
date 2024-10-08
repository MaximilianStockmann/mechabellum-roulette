// Features:
// 1. Enter tournament participants
// 2. Automatic tournament pairing
// 3. Randomize unit types allowed for use in match (pay attention to ground & air attack types)
// 4. Print tournament schedule

use core::fmt::{self, Formatter};
use core::panic;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::io;

enum Tools {
    Tournament,
    Roulette,
}

enum GameType {
    Single,
    Double,
    Ffa,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum UnitType {
    Air,
    Ground,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum UnitAttackType {
    Ground,
    AirAndGround,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Unit {
    id: i32,
    name: String,
    unit_type: UnitType,
    unit_attack_type: UnitAttackType,
    is_giant: bool,
    is_rare: bool,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn main() {
    print_welcome();
    match tool_choice() {
        Tools::Roulette => start_roulette(),
        Tools::Tournament => (),
    }
}

fn print_welcome() {
    println!("Welcome to Mechabellum Roulette!");
    println!("Please enter what you would like to do: ");
    println!("1. Plan tournament");
    println!("2. Get random units");
}

fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(error) => println!("Error occured: {error}"),
        Ok(_) => (),
    }
    let final_input = String::from(input.trim());
    return final_input;
}

fn tool_choice() -> Tools {
    let input = get_input();
    loop {
        match input.as_str() {
            "1" => return Tools::Tournament,
            "2" => return Tools::Roulette,
            _ => println!("Please enter a different choice"),
        }
    }
}

fn start_roulette() {
    println!("Choose game type: ");
    println!("1. 1v1");
    println!("2. 2v2");
    println!("3. FFA");
    let choice_string = get_input();

    let choice: GameType;
    match choice_string.as_str() {
        "1" => choice = GameType::Single,
        "2" => choice = GameType::Double,
        "3" => choice = GameType::Ffa,
        _ => {
            // TODO: This has to enter into a repeat of the prompt
            println!("Please enter a different choice");
            choice = GameType::Single;
        }
    };

    roulette(&choice);
}

fn roulette(choice: &GameType) {
    match choice {
        GameType::Single => execute_roulette(2),
        GameType::Double => execute_roulette(4),
        GameType::Ffa => execute_roulette(4),
    }
}

fn execute_roulette(player_number: usize) {
    let names = enter_names(&player_number);
    println!("{:?}", names);

    println!("Please enter amount of unit types allowed in match (1..25): ");

    // Convert input to i32
    let unit_number = (*get_input()).parse::<i32>().unwrap();

    let units = parse_unit_data().unwrap();

    for i in 0..player_number {
        randomize_and_print(&names[i], &unit_number, &units);
    }
}

fn randomize_and_print(player_name: &String, unit_number: &i32, unit_list: &Vec<Unit>) {
    println!("Randomized units for {}", player_name);
    let player_units = randomize_unit_types(&unit_number, &unit_list);
    for unit in player_units {
        println!("{}", unit)
    }

    //Print empty line for formatting purposes
    println!("");
}

fn enter_names(player_number: &usize) -> Vec<String> {
    let mut names = Vec::<String>::new();

    println!("One after the other, please enter all player names: ");

    for _ in 0..*player_number {
        let name = get_input();
        names.push(name);
    }
    names
}

// TODO: Adjust so there is an equal distribution of T1, T2, T3 units
fn randomize_unit_types(unit_type_number: &i32, units: &Vec<Unit>) -> Vec<Unit> {
    let mut rng = thread_rng();
    let final_units;

    loop {
        let mut selected_units = Vec::<Unit>::new();
        let mut unit_id_cache: Vec<i32> = Vec::<i32>::new();

        for _ in 0..*unit_type_number {
            let mut random_unit_id = rng.gen_range(1..=25);
            loop {
                if !unit_id_cache.contains(&random_unit_id) {
                    unit_id_cache.push(random_unit_id);
                    break;
                }
                random_unit_id = rng.gen_range(1..=25);
            }

            let random_unit = units.iter().find(|e| e.id == random_unit_id).unwrap();
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
        .any(|unit| unit.unit_attack_type == UnitAttackType::AirAndGround)
        || all_attack_types_present;

    all_attack_types_present
}

fn parse_unit_data() -> Result<Vec<Unit>, Box<dyn Error>> {
    let bytes = include_bytes!("../data/units.json");
    let s = std::str::from_utf8(bytes).unwrap();

    let units: Vec<Unit> = serde_json::from_str(&s)?;

    Ok(units)
}
