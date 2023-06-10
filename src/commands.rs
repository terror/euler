use {
  super::*, ai::AI_COMMAND, course::COURSE_COMMAND, help::HELP_COMMAND,
  problem::PROBLEM_COMMAND,
};

mod ai;
mod course;
mod help;
mod problem;

#[group]
#[commands(ai, course, help, problem)]
pub(crate) struct Commands;
