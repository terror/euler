use super::*;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
  msg
    .channel_id
    .send_message(&ctx.http, |m| {
      m.embed(|e| {
        e.title("Help")
          .description("Available commands:")
          .field("!ai [question]", "Query ChatGPT.", false)
          .field(
            "!course [code]",
            "Get information about a specific McGill course.",
            false,
          )
          .field(
            "!problem [difficulty]",
            "Get information about a random leetcode problem.",
            false,
          )
          .field("!help", "Display this help message.", false)
      })
    })
    .await?;

  Ok(())
}
