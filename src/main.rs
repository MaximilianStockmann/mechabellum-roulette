// Features:
// 1. Enter tournament participants
// 2. Automatic tournament pairing
// 3. Randomize unit types allowed for use in match (pay attention to ground & air attack types)
// 4. Print tournament schedule

use core::fmt::{self, Formatter};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{write, Display};
use std::fs::File;
use std::io;
use std::io::BufReader;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
enum UnitAttackType {
    Air,
    Ground,
    AirAndGround,
}

//TODO: Implement Display trait for Unit
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Unit {
    id: i32,
    name: String,
    unit_type: UnitType,
    // TODO: Problem: Techs can make units change attack types. How to deal?
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
    let input = get_input();
    choice(input);
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

fn choice(choice_string: String) {
    match choice_string.as_str() {
        "1" => (), // change this to later start the tournament planning
        "2" => start_roulette(),
        _ => println!("Please enter a different choice"),
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
        GameType::Single => roulette_single(),
        GameType::Double => roulette_double(),
        GameType::Ffa => roulette_ffa(),
    }
}

fn roulette_single() {
    let player_number = 2usize;
    let names = enter_names(&player_number);
    println!("{:?}", names);

    println!("Please enter amount of unit types allowed in match (1..25): ");

    // Convert input to i32
    let unit_type_number = (*get_input()).parse::<i32>().unwrap();

    let units = parse_unit_data().unwrap();

    for i in 0..player_number {
        println!("Randomized units for {}", names[i]);
        let player_units = randomize_unit_types(&unit_type_number, &units);
        for unit in player_units {
            println!("{}", unit)
        }

        //Print empty line for formatting purposes
        println!("");
    }
}

fn roulette_double() {
    // TODO: This is different to ffa in that we can limit the necessity of ground & air attack
    // types to teams rather than players
    let player_number = 4;
}

fn roulette_ffa() {
    let player_number = 4;
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

// TODO: Adjust randomization to take into account attack types
// TODO: Adjust so duplicate unit types are impossible
// TODO: Adjust so there is an equal distribution of T1, T2, T3 units
fn randomize_unit_types(unit_type_number: &i32, units: &Vec<Unit>) -> Vec<Unit> {
    let mut rng = thread_rng();

    // Get the parsed json data and then loop an amount of times equal to the unit_type_number
    // Add the randomized unit id to the vector each time, remove from the parsed json collection
    let mut selected_units = Vec::<Unit>::new();

    for _ in 0..*unit_type_number {
        // TODO: Check if 25 is included here
        let random_unit_id = rng.gen_range(1..25);
        let random_unit = units.iter().find(|e| e.id == random_unit_id).unwrap();
        selected_units.push(random_unit.clone());
    }
    //Choose random units from units

    selected_units
}

fn parse_unit_data() -> Result<Vec<Unit>, Box<dyn Error>> {
    // let file = File::open("data/units.json")?;
    let bytes = include_bytes!("../data/units.json");
    // let reader = BufReader::new(file);
    let s = std::str::from_utf8(bytes).unwrap();

    let units: Vec<Unit> = serde_json::from_str(&s)?;

    /* let units: Vec<Unit> = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(why) => panic!("{:?}", why),
    }; */

    Ok(units)
}
