use {
  chatgpt::prelude::*,
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
#[commands(ai, course, help, problem)]
struct Commands;

#[command]
async fn ai(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let question = args.remains().unwrap_or_default().trim().to_string();

  let client = ChatGPT::new(env::var("OPENAI_API_KEY")?)?;

  msg
    .reply(
      ctx,
      format!(
        "```{}```",
        client.send_message(question).await?.message().content
      ),
    )
    .await?;

  Ok(())
}

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
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
  msg
    .reply(
      ctx,
      "This bot supports the following commands:\n\
      `!ai [question]`: Asks a question to ChatGPT and returns the response.\n\
      `!course [course code]`: Returns information about the specified course.\n\
      `!problem [difficulty]`: Returns a randomly selected LeetCode problem. \
      The optional `difficulty` parameter can be 'easy', 'medium', 'hard', or 'all' (default).\n\
      `!help`: Displays this help message.",
    )
    .await?;

  Ok(())
}

#[command]
async fn problem(
  ctx: &Context,
  msg: &Message,
  mut args: Args,
) -> CommandResult {
  let difficulty = match args.single::<String>() {
    Ok(value) => value.to_lowercase(),
    Err(_) => String::from("all"),
  };

  let content = reqwest::get("https://leetcode.com/api/problems/all/")
    .await?
    .json::<serde_json::Value>()
    .await?;

  let problems = content["stat_status_pairs"]
    .as_array()
    .unwrap()
    .iter()
    .filter_map(|obj| {
      let difficulty_level = obj["difficulty"]["level"].as_i64()?;

      let problem = obj["stat"].clone();

      match difficulty.as_str() {
        "easy" if difficulty_level == 1 => Some(problem),
        "medium" if difficulty_level == 2 => Some(problem),
        "hard" if difficulty_level == 3 => Some(problem),
        "all" => Some(problem),
        _ => None,
      }
    })
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
