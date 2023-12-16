use clap::Subcommand;

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// run day 1 solution
    DAY1,
    /// run day 2 solution
    DAY2
}

pub const DEFAULT_COMMAND: Command = Command::DAY2;

impl Into<u32> for Command {
    fn into(self) -> u32 {
        match self {
            Self::DAY1 => 1,
            Self::DAY2 => 2,
        }
    }
}
