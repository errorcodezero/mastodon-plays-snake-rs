use std::env;
use crate::game::{Direction, Game};
use megalodon::{self};
use tokio;

pub mod game;

#[tokio::main]
async fn main() -> Result<(), megalodon::error::Error> {
    let instance = env::var("INSTANCE").unwrap();
    let access_token = env::var("ACCESS_TOKEN").unwrap();
    let _client = megalodon::generator(
        megalodon::SNS::Mastodon,
        instance,
        Some(access_token),
        None
    ).expect("Valid mastodon instance and access token.");

    let mut game = Game::new();
    game.setup();

    println!("{}", game.to_string());
    game.move_snake(Direction::Up);
    println!("{}", game.to_string());
    game.move_snake(Direction::Left);
    println!("{}", game.to_string());

    Ok(())
}
