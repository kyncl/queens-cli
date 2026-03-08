use clap::Parser;
use queens_cli::{app::App, args::Args, board::Board, board_selection::BoardSelection};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = Args::parse();
    color_eyre::install()?;
    let board = if let Some(board) = args.board {
        Some(Board::load_board(&board, None))
    } else {
        let terminal = ratatui::init();
        BoardSelection::new().run(terminal).await?
    };
    if let Some(board) = board {
        let terminal = ratatui::init();
        App::new(board).run(terminal).await?;
    }
    ratatui::restore();
    Ok(())
}
