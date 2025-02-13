use crate::unit::Unit;

pub mod roulette;

pub enum Tools {
    Tournament(GameType),
    Roulette(GameType),
}

impl Tools {
    pub fn new_singles_roulette(player_names: Vec<String>) -> Self {
        let players = Self::add_players(player_names);
        Tools::Roulette(GameType::Single(players))
    }

    pub fn new_doubles_roulette(player_names: Vec<String>) -> Self {
        let players = Self::add_players(player_names);
        Tools::Roulette(GameType::Double(players))
    }

    pub fn new_ffa_roulette(player_names: Vec<String>) -> Self {
        let players = Self::add_players(player_names);
        Tools::Roulette(GameType::FFA(players))
    }

    fn add_players(player_names: Vec<String>) -> Vec<Player> {
        let mut players = Vec::new();

        for player_name in player_names.iter() {
            players.push(Player {
                name: player_name.to_string(),
                assigned_units: Vec::new(),
            });
        }

        players
    }
}

pub enum GameType {
    Single(Vec<Player>),
    Double(Vec<Player>),
    FFA(Vec<Player>),
}

impl GameType {
    pub fn get_player_number(&self) -> u32 {
        match self {
            GameType::Single(_) => 2,
            GameType::Double(_) => 4,
            GameType::FFA(_) => 4,
        }
    }

    pub fn get_players(&self) -> &Vec<Player> {
        match self {
            GameType::Single(players) => players,
            GameType::Double(players) => players,
            GameType::FFA(players) => players,
        }
    }
}

pub struct Player {
    name: String,
    assigned_units: Vec<Unit>,
}
