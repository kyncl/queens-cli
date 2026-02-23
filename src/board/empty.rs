use crate::board::{Board, queen::toggle_queen};

pub fn toggle_empty(board: &mut Board, selected_pos: (u8, u8), auto_empty: bool) {
    if let Some(index) = board
        .empty_pos
        .iter()
        .position(|&e_pos| e_pos == selected_pos)
    {
        board.empty_pos.swap_remove(index);
    } else {
        if board.queen_pos.contains(&selected_pos) {
            toggle_queen(board, selected_pos, auto_empty);
        }
        board.empty_pos.push(selected_pos);
    }
}
