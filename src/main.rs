use crate::game::Game;
use game::Direction;
use megalodon::megalodon::{
    CredentialsFieldAttribute, GetAccountStatusesInputOptions, PollOptions, PostStatusInputOptions,
    UpdateCredentialsInputOptions,
};
use rand::prelude::SliceRandom;
use std::{env, time::Duration};
use tokio::time;

pub mod game;

#[tokio::main]
async fn main() -> Result<(), megalodon::error::Error> {
    let instance = env::var("INSTANCE").unwrap();
    let access_token = env::var("ACCESS_TOKEN").unwrap();
    let client = megalodon::generator(megalodon::SNS::Mastodon, instance, Some(access_token), None)
        .expect("Valid mastodon instance and access token.");
    let id = env::var("ID").unwrap();

    let mut game = Game::new();
    game.setup();

    let account = client.get_account(id).await?;

    let fields = account.json.fields;
    let mut backup_exists = false;

    for field in fields {
        if field.name == "_backup" {
            game.import(field.value);
            backup_exists = true;
        }
    }

    if !backup_exists {
        let opt = UpdateCredentialsInputOptions {
            fields_attributes: Some(vec![
                CredentialsFieldAttribute {
                    name: "Updates".to_string(),
                    value: "Every 30 Minutes".to_string(),
                },
                CredentialsFieldAttribute {
                    name: "Website".to_string(),
                    value: "https://errorcodezero.dev".to_string(),
                },
                CredentialsFieldAttribute {
                    name: "Source Code".to_string(),
                    value: "https://github.com/errorcodezero/mastodon-plays-snake-rs".to_string(),
                },
                CredentialsFieldAttribute {
                    name: "_backup".to_string(),
                    value: game.create_backup(),
                },
            ]),
            ..Default::default()
        };
        client.update_credentials(Some(&opt)).await?;
    }

    loop {
        let id = env::var("ID").unwrap();

        let directions = game.get_directions();
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
            .post_status(game.to_string(), Some(&post_options))
            .await?;
        time::sleep(Duration::from_secs(15)).await;
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
            let mut most_voted: Vec<String> = vec![];
            let _ = poll.options.iter().for_each(|x| {
                if let Some(votes) = x.votes_count {
                    if votes == max_votes {
                        max_votes = votes;
                        most_voted.push(x.title.clone());
                    }
                    if votes > max_votes {
                        max_votes = votes;
                        most_voted = vec![x.title.clone()];
                    }
                }
            });
            let most_voted = most_voted.choose(&mut rand::thread_rng());
            if let Some(most_voted) = most_voted {
                if most_voted == "⬆️" {
                    game.move_snake(Direction::Up);
                } else if most_voted == "⬇️" {
                    game.move_snake(Direction::Down);
                } else if most_voted == "⬅️" {
                    game.move_snake(Direction::Left);
                } else if most_voted == "➡️" {
                    game.move_snake(Direction::Right);
                } else {
                    game.move_snake(game.get_current_direction());
                }
            }
        } else {
            game.setup();
        }

        let opt = UpdateCredentialsInputOptions {
            fields_attributes: Some(vec![
                CredentialsFieldAttribute {
                    name: "Updates".to_string(),
                    value: "Every 30 Minutes".to_string(),
                },
                CredentialsFieldAttribute {
                    name: "Website".to_string(),
                    value: "https://errorcodezero.dev".to_string(),
                },
                CredentialsFieldAttribute {
                    name: "Source Code".to_string(),
                    value: "https://github.com/errorcodezero/mastodon-plays-snake-rs".to_string(),
                },
                CredentialsFieldAttribute {
                    name: "_backup".to_string(),
                    value: game.create_backup(),
                },
            ]),
            ..Default::default()
        };
        client.update_credentials(Some(&opt)).await?;
    }
}
