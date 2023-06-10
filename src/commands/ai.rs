use super::*;

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
