use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use rand::Rng;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn roll(
    ctx: Context<'_>,
    #[description = "The number of sides on the die (4,6,8,10,12,100)"] sides: u32,
    #[description = "The number of dice to roll"] num: u32,
    #[description = "The modifier to add to the roll"] modifier: i32,
) -> Result<(), Error>{
    // Validating the input
    let valid = [4,6,8,10,12,100];
    if !valid.contains(&sides){
        ctx.say("Invalid number of sides. Please choose from 4,6,8,10,12,100").await?;
        return Ok(());
    }
    let mut total: i32 = 0;
    let mut rolls = Vec::new();
    for _ in 0..num{
        let roll = rand::thread_rng().gen_range(1..=sides) as i32;
        rolls.push(roll);
        total = total + roll;
    }
    total = total + modifier;

    let rolls_str = rolls.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");;
    ctx.say(format!("Rolls: {}\nTotal: {}", rolls_str, total)).await?;
    Ok(())
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), roll()], // Add the command to the framework
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
