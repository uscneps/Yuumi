use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions};
use crate::commands::build;
use crate::types::{Data, Error};

/// start poise framework
pub async fn run() -> Result<(), Error> {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![build()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start().await.unwrap();
    Ok(())
}
