use super::*;

pub(crate) struct Course(pub(crate) Html);

unsafe impl Send for Course {}

impl Course {
  pub(crate) fn title(&self) -> Result<String> {
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

  pub(crate) fn description(&self) -> Result<String> {
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

  pub(crate) fn instructors(&self) -> Result<String> {
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

    for (term, full_term) in terms.join(" ").split(", ").map(|term| {
      (
        term.split(' ').take(1).collect::<String>(),
        term.to_string(),
      )
    }) {
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
    }

    if instructors.len() == 0 {
      return Ok(String::from(
        "There are no instructors associated with this course.",
      ));
    }

    let names = instructors
      .iter()
      .map(|instructor| format!("{} ({})", instructor.name, instructor.term))
      .collect::<Vec<String>>()
      .join(", ");

    if instructors.len() > 1 {
      let index = names.rfind(", ").unwrap();
      return Ok(format!("{} and {}.", &names[..index], &names[index + 2..]));
    }

    Ok(names)
  }
}
