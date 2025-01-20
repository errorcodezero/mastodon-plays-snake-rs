use crate::game::{Direction, Game};

pub mod game;

fn main() {
    let mut game = Game::new();
    game.setup();
    let mut dir = Direction::Down;

    dir = dir.get_valid_dir().0;

    println!("{:?}", dir);

    println!("{}", &game.to_string());
    game.inc_score();
    println!("Score: {}", &game.get_score());
}
