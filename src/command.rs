use clap::Subcommand;

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// run day_1 solution
    DAY1,
    DAY2
}

pub const DEFAULT_COMMAND: Command = Command::DAY1;

impl Into<u32> for Command {
    fn into(self) -> u32 {
        match self {
            Self::DAY1 => 1,
            Self::DAY2 => 2,
        }
    }
}
