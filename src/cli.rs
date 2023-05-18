use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    Dump(TwoFiles),
    Load(OneFile),
    Dync(OneFile),
}

#[derive(Debug, Args)]
struct OneFile {
    pub file: String,
}

#[derive(Debug, Args)]
struct TwoFiles {
    pub dict: String,
    pub dest: String,
}

pub fn parse_args() -> Arguments {
    let args = Cli::parse();
    match args.action {
        Action::Dump(TwoFiles { dict, dest }) => Arguments::Dump(dict, dest),
        Action::Load(OneFile { file }) => Arguments::Load(file),
        Action::Dync(OneFile { file }) => Arguments::Dync(file),
    }
}

pub enum Arguments {
    Dump(String, String),
    Load(String),
    Dync(String),
}
