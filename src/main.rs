use {
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

const MCGILL_TERM: &str = "2023-2024";

pub(crate) trait Select<'a> {
  fn select_single(&self, selector: &str) -> Result<ElementRef<'a>>;
  fn try_select_single(&self, selectors: Vec<&str>) -> Result<ElementRef<'a>>;
  fn select_optional(&self, selector: &str) -> Result<Option<ElementRef<'a>>>;
  fn select_many(&self, selector: &str) -> Result<Vec<ElementRef<'a>>>;
}

impl<'a> Select<'a> for ElementRef<'a> {
  fn select_single(&self, selector: &str) -> Result<ElementRef<'a>> {
    self
      .select(
        &Selector::parse(selector)
          .map_err(|error| anyhow!("Failed to parse selector: {:?}", error))?,
      )
      .next()
      .ok_or_else(|| anyhow!("Failed to select element"))
  }

  fn try_select_single(&self, selectors: Vec<&str>) -> Result<ElementRef<'a>> {
    selectors
      .iter()
      .map(|selector| self.select_single(selector))
      .find(|result| result.is_ok())
      .unwrap_or_else(|| {
        Err(anyhow!(
          "Failed to select element with selectors: {:?}",
          selectors
        ))
      })
  }

  fn select_optional(&self, selector: &str) -> Result<Option<ElementRef<'a>>> {
    Ok(
      self
        .select(
          &Selector::parse(selector).map_err(|error| {
            anyhow!("Failed to parse selector: {:?}", error)
          })?,
        )
        .next(),
    )
  }

  fn select_many(&self, selector: &str) -> Result<Vec<ElementRef<'a>>> {
    Ok(
      self
        .select(
          &Selector::parse(selector).map_err(|error| {
            anyhow!("Failed to parse selector: {:?}", error)
          })?,
        )
        .collect(),
    )
  }
}

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

struct CourseHtmlWrapper(Html);

unsafe impl Send for CourseHtmlWrapper {}

impl CourseHtmlWrapper {
  fn title(&self) -> Result<String> {
    Ok(
      self
        .0
        .root_element()
        .select_single("h1[id='page-title']")?
        .inner_html()
        .trim()
        .to_owned(),
    )
  }

  fn description(&self) -> Result<String> {
    let content = self
      .0
      .root_element()
      .select_single("div[class='node node-catalog clearfix']")?;

    Ok(
      content
        .select_single("div[class='content']")?
        .select_single("p")?
        .inner_html()
        .trim()
        .split(':')
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_owned(),
    )
  }

  fn extract_course_instructors(&self) -> Result<Vec<Instructor>> {
    let mut instructors = Vec::new();

    let catalog = self.0.root_element().try_select_single(vec![
      "div[class='node node-catalog clearfix']",
      "div[class='node node-catalog node-promoted clearfix']",
    ])?;

    let raw = catalog
      .select_single("p[class='catalog-terms']")?
      .inner_html();

    let terms = raw
      .trim()
      .split(' ')
      .skip(1)
      .filter(|entry| !entry.is_empty())
      .collect::<Vec<&str>>();

    let mut tokens = catalog
      .select_single("p[class='catalog-instructors']")?
      .inner_html()
      .trim()
      .split(' ')
      .skip(1)
      .collect::<Vec<&str>>()
      .join(" ");

    terms
      .join(" ")
      .split(", ")
      .map(|term| {
        (
          term.split(' ').take(1).collect::<String>(),
          term.to_string(),
        )
      })
      .for_each(|(term, full_term)| {
        if tokens.contains(&format!("({term})")) {
          let split = tokens.split(&format!("({term})")).collect::<Vec<&str>>();

          let inner = split[0]
            .split(';')
            .map(|s| {
              Instructor::default()
                .set_name(&s.trim().split(", ").collect::<Vec<&str>>())
                .set_term(&full_term)
            })
            .collect::<Vec<Instructor>>();

          if split.len() > 1 {
            tokens = split[1].trim().to_string();
          }

          instructors.extend(inner);
        }
      });

    Ok(instructors)
  }

  fn instructors(&self) -> Result<String> {
    let instructors = self.extract_course_instructors()?;

    let names: Vec<String> = instructors
      .iter()
      .map(|instructor| format!("{} ({})", instructor.name, instructor.term))
      .collect();

    let joined = names.join(", ");

    if names.len() > 1 {
      let index = joined.rfind(", ").unwrap();
      Ok(format!("{} and {}", &joined[..index], &joined[index + 2..]))
    } else {
      Ok(joined)
    }
  }
}

#[derive(Debug, Default)]
pub struct Instructor {
  pub name: String,
  pub term: String,
}

impl Instructor {
  pub fn set_name(self, parts: &[&str]) -> Self {
    Self {
      name: format!(
        "{} {}",
        parts.get(1).unwrap_or(&""),
        parts.first().unwrap_or(&"")
      ),
      ..self
    }
  }

  pub fn set_term(self, term: &str) -> Self {
    Self {
      term: term.to_owned(),
      ..self
    }
  }
}

#[command]
async fn course(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let course_code = args.single::<String>()?;

  let base_url = format!("https://www.mcgill.ca/study/{MCGILL_TERM}/courses");

  let parsed = Regex::new(r"([a-zA-Z]+)(\d+)")
    .unwrap()
    .replace(&course_code, "$1-$2");

  let course_html = CourseHtmlWrapper(Html::parse_fragment(
    reqwest::get(&format!("{}/{}", base_url, parsed))
      .await?
      .text()
      .await?
      .as_str(),
  ));

  msg
    .reply(
      ctx,
      format!(
        "**{}**\n{}\n*Taught by {}*",
        course_html.title()?,
        course_html.description()?,
        course_html.instructors()?,
      ),
    )
    .await?;

  Ok(())
}

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
