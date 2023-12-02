use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub part: PartSelector,

    #[arg(long)]
    pub real: bool,

    #[arg(long)]
    pub print_input: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PartSelector {
    PartOne,
    PartTwo,
}
