use std::io;
mod game_state;
mod logger;
mod player;

use game_state::GameState;
use logger::Logger;
use player::Player;

fn main() {
    // Initialize the logger
    let mut logger = Logger::new("debug.log").expect("Failed to initialize logger");
    logger
        .log("Starting the game")
        .expect("Failed to write log");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let player_number = input.chars().nth(10).unwrap();
    let player = Player::from_char(player_number).expect("Invalid player number");
    let mut game = GameState::new(player);

    loop {
        logger.log("New round").expect("Failed to write log");
        if let Err(e) = game.read_grid(&mut logger) {
            logger.log(&format!("Error reading grid: {}", e)).unwrap();
            return;
        }


        if let Err(e) = game.read_piece(&mut logger) {
            logger.log(&format!("Error reading piece: {}", e)).unwrap();
            return;
        }

        let position = game.choose_coordinates();
        logger.log(&format!("Chosen coordinates: x = {}, y = {}", position.x, position.y)).unwrap();
        println!("{} {}", position.x, position.y);
    }
}
