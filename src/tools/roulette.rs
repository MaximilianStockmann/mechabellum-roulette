use crate::tools::GameType;
use crate::unit::{self, Unit};

struct RouletteOptions {
    unit_count: u8,            // the amount of random units to generate for each player
    allow_rare_units: bool, // allows for the roulette to generate units that are in the rare unit pool for a player
    cost_balancing: bool, // makes sure all players have roughly the same total cost of their teams
    cost_balancing_range: u32, // sets the range in credits which the total cost of player teams can range from each other
    total_unit_cost: u32, // sets the total unit cost in case the option 'cost_balancing' is enabled
    allow_no_air: bool, // makes it possible for players to have no air defense in their randomized units
    invert_roulette: bool, // instead of generating allowed units, generates banned units
}

/*
pub fn execute_roulette(roulette_options: RouletteOptions, game_type: &GameType) {
    let units = unit::parse_unit_data().unwrap();

    println!(
        "Please enter amount of unit types allowed in match (1..{:?}): ",
        units.len()
    );

    // Convert input to i32
    let unit_number = (*get_input()).parse::<i32>().unwrap();

    for i in 0..player_number {
        randomize_and_print(&names[i], &unit_number, &units);
    }
} */

pub fn execute_default_roulette(game_type: &GameType) {
    let units = unit::parse_unit_data().unwrap();

    // Convert input to i32
    let unit_number = 5;

    for player in game_type.get_players().iter() {
        randomize_and_print(&player.name, &unit_number, &units);
    }
}

fn randomize_and_print(player_name: &String, unit_number: &i32, unit_list: &Vec<Unit>) {
    println!("Randomized units for {}", player_name);
    let player_units = unit::randomize_unit_types(&unit_number, &unit_list);
    for unit in player_units {
        println!("{}", unit)
    }

    //Print empty line for formatting purposes
    println!("");
}
