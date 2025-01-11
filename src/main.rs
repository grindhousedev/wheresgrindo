mod bot;

use bot::Bot;

use eyre::{eyre, Result as EyreResult};
use twitch_eventsub::TwitchKeys;

fn main() -> EyreResult<()> {
    let keys = TwitchKeys::from_secrets_env()
        .map_err(|err| eyre!("Failed to obtain secret keys: {:?}", err))?;

    let mut bot = Bot::new(keys)?;

    bot.run()?;

    Ok(())
}
