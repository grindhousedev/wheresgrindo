use std::time::Duration;

use eyre::{eyre, Result as EyreResult};
use twitch_eventsub::{Event, ResponseType, Subscription, TwitchEventSubApi, TwitchKeys};

pub struct Bot {
    api: TwitchEventSubApi,
}

impl Bot {
    pub fn new(keys: TwitchKeys) -> EyreResult<Self> {
        let api = TwitchEventSubApi::builder(keys)
            .set_redirect_url("http://localhost:3000")
            .generate_new_token_if_insufficent_scope(true)
            .generate_new_token_if_none(true)
            .generate_access_token_on_expire(true)
            .auto_save_load_created_tokens(".user_token.env", ".refresh_token.env")
            .add_subscriptions(vec![Subscription::ChatMessage])
            .build()
            .map_err(|err| eyre!("Failed to instantiate Twitch API: {:?}", err))?;

        Ok(Self { api })
    }

    pub fn run(&mut self) -> EyreResult<()> {
        loop {
            let Some(response) = self.api.receive_single_message(Duration::from_millis(50)) else {
                continue;
            };

            match response {
                ResponseType::Event(Event::ChatMessage(msg_data)) => {
                    let msg = msg_data.message.text;
                    let user = msg_data.chatter.name;
                    let badges = msg_data.badges;

                    eprintln!("{} said: {:?}", user, msg);

                    let elevated = badges
                        .iter()
                        .any(|b| ["broadcaster", "moderator"].contains(&b.set_id.as_str()));

                    if !elevated {
                        continue;
                    }

                    match msg.as_str() {
                        "!ping" => {
                            self.send_chat_message("pong!")?;
                        }
                        "!discord" => {
                            self.send_chat_message("Join the community at join.grindhouse.dev")?;
                        }
                        _ => {}
                    }
                }
                ResponseType::Close => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}

impl Bot {
    fn send_chat_message(&mut self, msg: &str) -> EyreResult<()> {
        let _ = self
            .api
            .send_chat_message(msg)
            .map_err(|err| eyre!("Faield to send chat message: {:?}", err))?;

        Ok(())
    }
}
