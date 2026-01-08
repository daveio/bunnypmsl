/// Bindings command handler
///
/// This command demonstrates the multi-binding feature.
/// Both "bindings" and "list" will trigger this command.
///
/// Examples:
/// - bindings -> /bindings
/// - list -> /bindings
use crate::commands::bunnypmsl_command::{BunnypmslCommand, BunnypmslCommandInfo};

pub struct BindingsCommand;

impl BunnypmslCommand for BindingsCommand {
    const BINDINGS: &'static [&'static str] = &[
        "bindings", "commands", "list", "bunny", "cmd", "cmds", "help",
    ];

    fn process_args(_args: &str) -> String {
        "/bindings".to_string()
    }

    fn get_info() -> BunnypmslCommandInfo {
        BunnypmslCommandInfo {
            bindings: Self::BINDINGS.iter().map(|s| s.to_string()).collect(),
            description: "View all Bunnypmsl command bindings in a web portal".to_string(),
            example: "bindings".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bindings_command() {
        assert_eq!(BindingsCommand::process_args("bindings"), "/bindings");
    }

    #[test]
    fn test_commands_command() {
        assert_eq!(BindingsCommand::process_args("commands"), "/bindings");
    }

    #[test]
    fn test_bunny_command() {
        assert_eq!(BindingsCommand::process_args("bunny"), "/bindings");
    }

    #[test]
    fn test_cmd_command() {
        assert_eq!(BindingsCommand::process_args("cmd"), "/bindings");
    }

    #[test]
    fn test_cmds_command() {
        assert_eq!(BindingsCommand::process_args("cmds"), "/bindings");
    }

    #[test]
    fn test_help_command() {
        assert_eq!(BindingsCommand::process_args("help"), "/bindings");
    }
}
