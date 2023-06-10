use super::*;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
  msg
    .reply(
      ctx,
      "```Available commands:\n\
      !ai [question]: Query ChatGPT.\n\
      !course [code]: Get information about a specific McGill course.\n\
      !problem [difficulty]: Get information about a random leetcode problem.\n\
      !help: Display this help message.```",
    )
    .await?;

  Ok(())
}
