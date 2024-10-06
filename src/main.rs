// Features: 
// 1. Enter tournament participants
// 2. Automatic tournament pairing
// 3. Randomize unit types allowed for use in match (pay attention to ground & air attack types)
// 4. Print tournament schedule
use std::io;
use rand::{thread_rng, Rng};

enum GameType {
    Single,
    Double,
    Ffa,
}

enum UnitType {
    Air,
    Ground
}

enum UnitAttackType {
    Air,
    Ground,
    AirAndGround
}

struct Unit {
    id: i32,
    name: String,
    unit_type: UnitType,
    // TODO: Problem: Techs can make units change attack types. How to deal?
    unit_attack_type: UnitAttackType
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
        Ok(_) => ()
    }
    let final_input = String::from(input.trim());
    return final_input;
}

fn choice(choice_string: String) {
    match choice_string.as_str() {
        "1" => (), // change this to later start the tournament planning
        "2" => start_roulette(),
        _ => println!("Please enter a different choice")
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
        GameType::Ffa => roulette_ffa()
    }
}

fn roulette_single() {
    let player_number = 2;
    let names = enter_names(&player_number);
    println!("{:?}", names);

    println!("Please enter amount of unit types allowed in match (1..25): ");
    let unit_type_number = get_input();

}

fn roulette_double() {
    // TODO: This is different to ffa in that we can limit the necessity of ground & air attack
    // types to teams rather than players
    let player_number = 4;

}

fn roulette_ffa() {
    let player_number = 4;

}

fn enter_names(player_number: &i32) -> Vec<String> {
    let mut names = Vec::<String>::new();

    println!("One after the other, please enter all player names: ");

    for _ in 0..*player_number {
        let name = get_input();
        names.push(name);
    }
    names
}

fn randomize_unit_types(unit_type_number: &i32, unit_type_list: [UnitType; 25]) -> Vec<UnitType> {
    let mut rng = thread_rng();

    // TODO: Check if 25 is included here
    rng.gen_range(1..25);
}
