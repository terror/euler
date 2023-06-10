#[derive(Debug, Default)]
pub(crate) struct Instructor {
  pub name: String,
  pub term: String,
}

impl Instructor {
  pub(crate) fn set_name(self, parts: &[&str]) -> Self {
    Self {
      name: format!(
        "{} {}",
        parts.get(1).unwrap_or(&""),
        parts.first().unwrap_or(&"")
      ),
      ..self
    }
  }

  pub(crate) fn set_term(self, term: &str) -> Self {
    Self {
      term: term.to_owned(),
      ..self
    }
  }
}
