/// Google Slides command handler
/// Supports: gslides -> redirects to Google Slides
use crate::commands::bunnypmsl_command::{BunnypmslCommand, BunnypmslCommandInfo};

pub struct GoogleSlidesCommand;

impl BunnypmslCommand for GoogleSlidesCommand {
    const BINDINGS: &'static [&'static str] = &["gslides"];

    fn process_args(_args: &str) -> String {
        "https://docs.google.com/presentation/u/0/".to_string()
    }

    fn get_info() -> BunnypmslCommandInfo {
        BunnypmslCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Google Slides".to_string(),
            example: "gslides".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_slides_command() {
        assert_eq!(
            GoogleSlidesCommand::process_args("gslides"),
            "https://docs.google.com/presentation/u/0/"
        );
    }

    #[test]
    fn test_google_slides_command_with_args() {
        assert_eq!(
            GoogleSlidesCommand::process_args("gslides some args"),
            "https://docs.google.com/presentation/u/0/"
        );
    }
}
