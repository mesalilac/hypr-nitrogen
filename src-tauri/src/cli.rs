use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long)]
    pub restore: bool,
}
