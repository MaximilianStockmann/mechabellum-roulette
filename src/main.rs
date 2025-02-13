// Features:
// 1. Enter tournament participants
// 2. Automatic tournament pairing
// 3. Randomize unit types allowed for use in match (pay attention to ground & air attack types)
// 4. Print tournament schedule

use mechabellum_roulette::tools::roulette;
use mechabellum_roulette::tools::Tools;
use std::io;
use std::process::exit;

fn main() {
    print_welcome();
    loop {
        tool_choice();
    }
}

fn print_welcome() {
    println!("Welcome to Mechabellum Roulette!");
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

fn tool_choice() {
    println!("Please enter what you would like to do: ");
    println!("1. Plan tournament");
    println!("2. Get random units");
    println!("q. Quit");

    let tool = match get_input().as_str() {
        "1" => None, // Not implemented yet
        "2" => roulette_choice(),
        "q" => {
            println!("Exiting program, goodbye!");
            exit(0);
        }
        _ => {
            println!("Please enter a different choice");
            None
        }
    };

    if let Some(selected_tool) = tool {
        match selected_tool {
            Tools::Roulette(game_type) => roulette::execute_default_roulette(&game_type),
            Tools::Tournament(_) => (), // Not implemented yet
        }
        return;
    }
}

fn roulette_choice() -> Option<Tools> {
    println!("Choose game type: ");
    println!("1. 1v1");
    println!("2. 2v2");
    println!("3. FFA");
    let choice_string = get_input();

    let choice: Option<Tools>;
    match choice_string.as_str() {
        "1" => choice = Some(Tools::new_singles_roulette(enter_names(2))),
        "2" => choice = Some(Tools::new_doubles_roulette(enter_names(4))),
        "3" => choice = Some(Tools::new_ffa_roulette(enter_names(4))),
        _ => {
            // TODO: This has to enter into a repeat of the prompt
            println!("Please enter a different choice");
            choice = None
        }
    }

    choice
}

fn enter_names(player_count: u32) -> Vec<String> {
    let mut names = Vec::<String>::new();

    println!("One after the other, please enter all player names: ");

    for _ in 0..player_count {
        let name = get_input();
        names.push(name);
    }

    names
}
