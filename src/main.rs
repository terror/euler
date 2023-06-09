use {
  dotenv::dotenv,
  log::info,
  rand::seq::SliceRandom,
  serenity::{
    async_trait,
    framework::standard::{
      macros::{command, group},
      Args, CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
  },
  std::{env, process},
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, ready: Ready) {
    info!("{} is connected!", ready.user.name);
  }
}

#[group]
#[commands(course, problem)]
struct Commands;

#[command]
async fn course(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let course_code = args.single::<String>()?;

  // TODO: just call the mcgill.courses public api :)
  msg
    .reply(ctx, format!("You asked for course: {}", course_code))
    .await?;

  Ok(())
}

#[command]
async fn problem(ctx: &Context, msg: &Message) -> CommandResult {
  let content = reqwest::get("https://leetcode.com/api/problems/all/")
    .await?
    .json::<serde_json::Value>()
    .await?;

  let problems = content["stat_status_pairs"]
    .as_array()
    .unwrap()
    .iter()
    .map(|obj| obj.get("stat").unwrap().clone())
    .collect::<Vec<serde_json::Value>>();

  let problem = problems.choose(&mut rand::thread_rng()).unwrap();

  msg
    .reply(
      ctx,
      format!(
        "https://leetcode.com/problems/{}",
        problem["question__title_slug"].as_str().unwrap()
      ),
    )
    .await?;

  Ok(())
}

async fn run() -> Result {
  Ok(
    Client::builder(
      &env::var("DISCORD_BOT_TOKEN")?,
      GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .framework(
      StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&COMMANDS_GROUP),
    )
    .await?
    .start()
    .await?,
  )
}

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() {
  dotenv().ok();

  env_logger::init();

  if let Err(error) = run().await {
    println!("error: {error}");
    process::exit(1);
  }
}
