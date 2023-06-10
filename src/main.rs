use {
  crate::{
    commands::COMMANDS_GROUP, course::Course, handler::Handler,
    instructor::Instructor, select::Select,
  },
  anyhow::anyhow,
  chatgpt::prelude::*,
  dotenv::dotenv,
  log::info,
  rand::seq::SliceRandom,
  regex::Regex,
  scraper::{ElementRef, Html, Selector},
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

mod commands;
mod course;
mod handler;
mod instructor;
mod select;

const MCGILL_TERM: &str = "2023-2024";

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
