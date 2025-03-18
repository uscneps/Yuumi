use crate::get_build_api;
use poise::Context;
use crate::types::{Data, Error};

/// /build or !build
#[poise::command(slash_command, prefix_command)]
pub async fn build(
    ctx: Context<'_, Data, Error>,
    #[description = "Selected champ"] champion: String,
) -> Result<(), Error> {

    let response = get_build_api::get_build(&champion).await
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            "Error".to_string()
        });

    ctx.say(response).await?;
    Ok(())
}
