use super::*;

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
