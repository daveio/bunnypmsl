/// Google Sheets command handler
/// Supports: gsheets -> redirects to Google Sheets
use crate::commands::bunnypmsl_command::{BunnypmslCommand, BunnypmslCommandInfo};

pub struct GoogleSheetsCommand;

impl BunnypmslCommand for GoogleSheetsCommand {
    const BINDINGS: &'static [&'static str] = &["gsheets"];

    fn process_args(_args: &str) -> String {
        "https://docs.google.com/spreadsheets/u/0/".to_string()
    }

    fn get_info() -> BunnypmslCommandInfo {
        BunnypmslCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "Navigate to Google Sheets".to_string(),
            example: "gsheets".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_sheets_command() {
        assert_eq!(
            GoogleSheetsCommand::process_args("gsheets"),
            "https://docs.google.com/spreadsheets/u/0/"
        );
    }

    #[test]
    fn test_google_sheets_command_with_args() {
        assert_eq!(
            GoogleSheetsCommand::process_args("gsheets some args"),
            "https://docs.google.com/spreadsheets/u/0/"
        );
    }
}
