/// Go documentation command handler
/// Supports:
/// - godocs -> https://go.dev/doc/
use crate::commands::bunnypmsl_command::{BunnypmslCommand, BunnypmslCommandInfo};

pub struct GodocsCommand;

impl BunnypmslCommand for GodocsCommand {
    const BINDINGS: &'static [&'static str] = &["godocs"];

    fn process_args(_args: &str) -> String {
        // Always redirect to Go documentation
        "https://go.dev/doc/".to_string()
    }

    fn get_info() -> BunnypmslCommandInfo {
        BunnypmslCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Go language documentation".to_string(),
            example: "godocs".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_godocs_command() {
        assert_eq!(GodocsCommand::process_args("godocs"), "https://go.dev/doc/");
        assert_eq!(
            GodocsCommand::process_args("godocs anything"),
            "https://go.dev/doc/"
        );
    }
}
