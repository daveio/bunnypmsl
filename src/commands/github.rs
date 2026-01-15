/// GitHub command handler
/// Supports: gh, gh [user], gh [user/repo]
use crate::commands::bunnypmsl_command::{BunnypmslCommand, BunnypmslCommandInfo};
use crate::utils::url_encoding::build_path_url;

pub struct GitHubCommand;

impl BunnypmslCommand for GitHubCommand {
  const BINDINGS: &'static [&'static str] = &["gh"];

  fn process_args(args: &str) -> String {
    let query = Self::get_command_args(args);
    if query.is_empty() {
      "https://github.com".to_string()
    } else {
      build_path_url("https://github.com", query)
    }
  }

  fn get_info() -> BunnypmslCommandInfo {
    BunnypmslCommandInfo {
      bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
      description: "Navigate to GitHub repositories".to_string(),
      example: "gh daveio/bunnypmsl".to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_github_command_base() {
    assert_eq!(GitHubCommand::process_args("gh"), "https://github.com");
  }

  #[test]
  fn test_github_command_user() {
    assert_eq!(
      GitHubCommand::process_args("gh daveio"),
      "https://github.com/daveio"
    );
  }

  #[test]
  fn test_github_command_repo() {
    assert_eq!(
      GitHubCommand::process_args("gh daveio/bunnypmsl"),
      "https://github.com/daveio/bunnypmsl"
    );
  }
}
