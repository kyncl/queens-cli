use crate::board::{Board, queen::toggle_queen};

/// Swaps between none -> empty -> queen and back
pub fn toggle_swap(board: &mut Board, selected_pos: (u8, u8), auto_empty: bool) {
    let pos = selected_pos;
    if board.queen_pos.contains(&pos) {
        toggle_queen(board, pos, auto_empty);
    } else if board.empty_pos.contains(&pos) {
        board.empty_pos.retain(|&p| p != pos);
        toggle_queen(board, pos, auto_empty);
    } else {
        board.empty_pos.push(pos);
    }
}
