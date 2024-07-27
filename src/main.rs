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
    #[description = "The number of sides on the die (4,6,8,10,12,20,100)"] sides: u32,
    #[description = "The number of dice to roll"] num: u32,
    #[description = "The modifier to add to the roll"] modifier: i32,
) -> Result<(), Error>{
    // Validating the input
    let valid = [4,6,8,10,12,20,100];
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
    let user_name = &ctx.author().name;
    let rolls_str = rolls.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
    if num == 1{
        let message = format!(r#"
{} rolled a {} sided die with a modifer of {},
The roll is {}
The total is {}"#, user_name, sides, modifier, total-modifier,total);
        ctx.say(message).await?;
        return Ok(());
    }
    let message =format!(r#"
{} rolled {}d{} with a modifer of {},
The following were rolled {}
The total is {}"#, user_name, num, sides, modifier, rolls_str, total);


    ctx.say(message).await?;
    Ok(())
}

// Beginning of Frog Command
#[poise::command(slash_command)]
async fn frog(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("A Frog will be here soon 🐸").await?;
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
            commands: vec![hello(), roll(), frog()], // Add the command to the framework
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
