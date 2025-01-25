use crate::game::Game;
use game::Direction;
use megalodon::megalodon::{GetAccountStatusesInputOptions, PollOptions, PostStatusInputOptions};
use std::{env, time::Duration};
use tokio::time;

pub mod game;

#[tokio::main]
async fn main() -> Result<(), megalodon::error::Error> {
    let instance = env::var("INSTANCE").unwrap();
    let access_token = env::var("ACCESS_TOKEN").unwrap();
    let client = megalodon::generator(megalodon::SNS::Mastodon, instance, Some(access_token), None)
        .expect("Valid mastodon instance and access token.");

    let mut game = Game::new();
    game.setup();

    loop {
        let directions = game.get_directions();
        let id = env::var("ID").unwrap();
        let mut poll_choices = vec![
            directions.0.get_emoji(),
            directions.1.get_emoji(),
            directions.2.get_emoji(),
        ];
        if let Some(i) = directions.3 {
            poll_choices.push(i.get_emoji());
        }
        let post_options: PostStatusInputOptions = PostStatusInputOptions {
            poll: Some(PollOptions {
                options: poll_choices,
                expires_in: Some(1800),
                hide_totals: Some(false),
                multiple: Some(false),
            }),
            language: Some("en".to_string()),
            ..Default::default()
        };
        let _post = client
            .post_status(
                game.to_string() + "\n\n" + "Score: " + &game.get_score().to_string(),
                Some(&post_options),
            )
            .await?;
        time::sleep(Duration::from_secs(30)).await;
        let get_options = GetAccountStatusesInputOptions {
            limit: Some(1),
            ..Default::default()
        };
        let posts = client.get_account_statuses(id, Some(&get_options)).await?;
        let posts = posts.json;
        // .clone() is only used since it complains about the Copy trait not being implemented but
        // it shouldn't be a huge hit to perf
        let poll = posts[0].poll.clone();
        if let Some(poll) = poll {
            let mut max_votes = 0;
            let mut most_voted = "".to_string();
            let _ = poll.options.iter().for_each(|x| {
                if let Some(votes) = x.votes_count {
                    if votes > max_votes {
                        max_votes = votes;
                        // again with the .clone()
                        most_voted = x.title.clone();
                    }
                }
            });
            if most_voted == "⬆️" {
                game.move_snake(Direction::Up);
            } else if most_voted == "⬇️" {
                game.move_snake(Direction::Down);
            } else if most_voted == "⬅️" {
                game.move_snake(Direction::Left);
            } else if most_voted == "➡️" {
                game.move_snake(Direction::Right);
            } else {
                if let Some(i) = game.get_current_direction() {
                    game.move_snake(i);
                } else {
                    game.move_snake(Direction::Up);
                }
            }
        } else {
            game.setup();
        }
    }
}
