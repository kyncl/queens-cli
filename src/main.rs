use clap::Parser;
use queens_cli::{app::App, args::Args};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = Args::parse();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let board = if let Some(board) = args.board {
        board
    } else {
        // todo!("Add selection of board");
        String::new()
    };
    let result = App::new(board).run(terminal).await;
    ratatui::restore();
    result
}
