use crate::game::Game;

pub mod game;

fn main() {
    let mut game = Game::new();
    game.put(0, 0, game::Block::Food);
    game.put(1, 1, game::Block::SnakeHead);
    game.put(2, 2, game::Block::SnakeBody);

    println!("{}", &game.to_string());
}
