use clap::Subcommand;

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// run day 1 solution
    DAY1,
    /// run day 2 solution
    DAY2,
    DAY2PART2
}

pub const DEFAULT_COMMAND: Command = Command::DAY2PART2;

impl Into<u32> for Command {
    fn into(self) -> u32 {
        match self {
            Self::DAY1 => 1,
            Self::DAY2 => 2,
            Self::DAY2PART2 => 2,
        }
    }
}
