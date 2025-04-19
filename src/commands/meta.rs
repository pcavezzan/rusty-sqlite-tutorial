use crate::commands::{CommandError, Execute, ExecutionError, TryFromStr};

#[derive(Debug, PartialEq)]
pub enum MetaCommand {
    Exit
}

impl TryFromStr for MetaCommand {
    type Error = CommandError;

    fn try_from_str(command: &str) -> Result<Option<Self>, Self::Error> {
        match command {
            ".exit" => Ok(Some(MetaCommand::Exit)),
            _ => Ok(None)
        }
    }
}

impl Execute for MetaCommand {
    fn execute(&self) -> Result<(), ExecutionError> {
        match self {
            MetaCommand::Exit => {
                std::process::exit(0);
            }
        }
    }
}

#[test]
fn test_parse_meta_command() {
    assert_eq!(
        MetaCommand::try_from_str(".exit"),
        Ok(Some(MetaCommand::Exit))
    );
    assert_eq!(MetaCommand::try_from_str("unknown command"), Ok(None));
}
