use crate::game::{Direction, Game};

pub mod game;

fn main() {
    let mut game = Game::new();
    game.setup();

    println!("{}", game.to_string());
    game.move_snake(Direction::Up);
    println!("{}", game.to_string());
    game.move_snake(Direction::Left);
    println!("{}", game.to_string());
}
