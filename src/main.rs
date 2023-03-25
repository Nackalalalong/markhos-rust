use crate::game::Game;

pub mod board;
pub mod game;

fn main() {
    let mut game = Game::new();
    
    while true {
        game.next_turn();
    }
}
