use super::*;

#[command]
async fn ai(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let question = args.rest().trim().to_string();

  if question.is_empty() {
    msg
      .reply(ctx, "Please provide a question after the command.")
      .await?;
    return Ok(());
  }

  let client = async_openai::Client::new();

  let request = CreateChatCompletionRequestArgs::default()
    .model("gpt-4")
    .messages([ChatCompletionRequestMessage::User(
      ChatCompletionRequestUserMessage {
        content: ChatCompletionRequestUserMessageContent::Text(question),
        name: None,
      },
    )])
    .build()?;

  match client.chat().create(request).await {
    Ok(response) => {
      if let Some(choice) = response.choices.first() {
        if let Some(content) = &choice.message.content {
          msg.reply(ctx, content).await?;
        } else {
          msg
            .reply(ctx, "Sorry, I couldn't generate a response.")
            .await?;
        }
      } else {
        msg
          .reply(ctx, "Sorry, I couldn't generate a response.")
          .await?;
      }
    }
    Err(_) => {
      msg
        .reply(ctx, "Sorry, there was an error processing your request.")
        .await?;
    }
  }

  Ok(())
}
