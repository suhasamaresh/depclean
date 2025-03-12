use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A tool to analyze and optimize project dependencies")]
pub struct Cli{
    /// The path to the Cargo.toml file
    #[arg(short, long, default_value = "Cargo.lock")]
    pub lockfile: String,

    ///Activate auto fix mode
    #[arg(long)]
    pub fix: bool,

    
}