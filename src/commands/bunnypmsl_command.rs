use serde::Serialize;

/// Information about a registered command binding
#[derive(Clone, Serialize)]
pub struct BunnypmslCommandInfo {
  pub bindings: Vec<String>,
  pub description: String,
  pub example: String,
}

/// Bunnypmsl Command trait that all URL builders must implement
pub trait BunnypmslCommand {
  /// All command strings that trigger this binding (e.g., ["gh", "github"])
  const BINDINGS: &'static [&'static str];

  /// Process the command arguments and return the appropriate URL
  fn process_args(args: &str) -> String;

  /// Get the command portion from the full arguments string
  fn get_command_args(args: &str) -> &str {
    // Check if args starts with any of the bindings
    for binding in Self::BINDINGS {
      if args.split_whitespace().next() == Some(*binding) {
        return if args.len() <= binding.len() {
          ""
        } else {
          args[binding.len()..].trim_start()
        };
      }
    }
    args
  }

  /// Check if this binding matches the given command
  fn matches_command(command: &str) -> bool {
    Self::BINDINGS.contains(&command)
  }

  /// Get information about this command (description and examples)
  fn get_info() -> BunnypmslCommandInfo;
}

#[cfg(test)]
mod tests {
  use super::*;

  // Mock command for testing
  struct TestCommand;

  impl BunnypmslCommand for TestCommand {
    const BINDINGS: &'static [&'static str] = &["test", "t"];

    fn process_args(args: &str) -> String {
      let query = Self::get_command_args(args);
      if query.is_empty() {
        "https://test.com".to_string()
      } else {
        format!("https://test.com/search?q={}", query)
      }
    }

    fn get_info() -> BunnypmslCommandInfo {
      BunnypmslCommandInfo {
        bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
        description: "Test command".to_string(),
        example: "test query".to_string(),
      }
    }
  }

  #[test]
  fn test_bunnypmsl_command_get_command_args() {
    assert_eq!(TestCommand::get_command_args("test"), "");
    assert_eq!(TestCommand::get_command_args("test hello"), "hello");
    assert_eq!(
      TestCommand::get_command_args("test hello world"),
      "hello world"
    );
  }

  #[test]
  fn test_bunnypmsl_command_matches_command() {
    assert!(TestCommand::matches_command("test"));
    assert!(TestCommand::matches_command("t"));
    assert!(!TestCommand::matches_command("other"));
  }

  #[test]
  fn test_bunnypmsl_command_process_args() {
    assert_eq!(TestCommand::process_args("test"), "https://test.com");
    assert_eq!(TestCommand::process_args("t"), "https://test.com");
    assert_eq!(
      TestCommand::process_args("test hello"),
      "https://test.com/search?q=hello"
    );
    assert_eq!(
      TestCommand::process_args("t hello"),
      "https://test.com/search?q=hello"
    );
  }
}
