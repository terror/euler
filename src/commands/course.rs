use super::*;

#[command]
async fn course(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let course_code = args.single::<String>()?;

  let base_url = format!("https://www.mcgill.ca/study/{MCGILL_TERM}/courses");

  let url = format!(
    "{}/{}",
    base_url,
    Regex::new(r"([a-zA-Z]+)(\d+)")?.replace(&course_code, "$1-$2")
  );

  let course = Course(Html::parse_fragment(
    reqwest::get(&url).await?.text().await?.as_str(),
  ));

  let title = course.title()?;
  let description = course.description()?;
  let instructors = course.instructors()?;

  msg
    .channel_id
    .send_message(ctx, |m| {
      m.embed(|e| {
        e.title(title)
          .description(description)
          .field("Instructors", instructors, true)
          .url(url)
      })
    })
    .await?;

  Ok(())
}
