/// Perplexity command handler
/// Supports: p, p [query]
use crate::commands::bunnypmsl_command::{BunnypmslCommand, BunnypmslCommandInfo};
use crate::utils::url_encoding::build_search_url;

pub struct PerplexityCommand;

impl BunnypmslCommand for PerplexityCommand {
  const BINDINGS: &'static [&'static str] = &["p"];

  fn process_args(args: &str) -> String {
    let query = Self::get_command_args(args);
    if query.is_empty() {
      "https://perplexity.ai".to_string()
    } else {
      build_search_url("https://perplexity.ai/search", "q", query)
    }
  }

  fn get_info() -> BunnypmslCommandInfo {
    BunnypmslCommandInfo {
      bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
      description: "Ask Perplexity".to_string(),
      example: "p why is the sky blue".to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_perplexity_command_base() {
    assert_eq!(
      PerplexityCommand::process_args("p"),
      "https://perplexity.ai"
    );
  }

  #[test]
  fn test_perplexity_command_query() {
    assert_eq!(
      PerplexityCommand::process_args("p why is the sky blue"),
      "https://perplexity.ai/search?q=why%20is%20the%20sky%20blue"
    );
  }
}
