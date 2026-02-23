use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "queens-cli",
    author = "Kyncl",
    version,
    about = "Logical game inspired by Queens game from LinkedIn, but in CLI"
)]
pub struct Args {
    /// Board that will be loaded into
    #[arg(short, long)]
    pub board: Option<String>,
}
