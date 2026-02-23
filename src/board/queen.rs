use crate::board::Board;

pub fn toggle_queen(board: &mut Board, selected_pos: (u8, u8), auto_empty: bool) {
    if board.queen_pos.contains(&selected_pos) {
        board.queen_pos.retain(|&p| p != selected_pos);
        if auto_empty {
            let (qx, qy) = selected_pos;
            let (width, height) = (board.size.0, board.size.1);
            let mut remove_one_mark = |p: (u8, u8)| {
                if let Some(index) = board.empty_pos.iter().position(|&x| x == p) {
                    board.empty_pos.remove(index);
                }
            };
            for i in 0..width {
                if (i, qy) != selected_pos {
                    remove_one_mark((i, qy));
                }
            }
            for j in 0..height {
                if (qx, j) != selected_pos {
                    remove_one_mark((qx, j));
                }
            }
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = qx as i16 + dx;
                    let ny = qy as i16 + dy;
                    if nx >= 0 && nx < width as i16 && ny >= 0 && ny < height as i16 {
                        remove_one_mark((nx as u8, ny as u8));
                    }
                }
            }
            if let Some(region) = board.regions.iter().find(|r| r.contains(&selected_pos)) {
                for &p in region {
                    if p != selected_pos {
                        remove_one_mark(p);
                    }
                }
            }
        }
    } else {
        board.queen_pos.push(selected_pos);
        if auto_empty {
            let (qx, qy) = selected_pos;
            let (width, height) = board.size;
            for i in 0..width {
                let row_p = (i, qy);
                if row_p != selected_pos {
                    board.empty_pos.push(row_p);
                }
            }
            for j in 0..height {
                let col_p = (qx, j);
                if col_p != selected_pos {
                    board.empty_pos.push(col_p);
                }
            }
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = qx as i16 + dx;
                    let ny = qy as i16 + dy;
                    if nx >= 0 && nx < width as i16 && ny >= 0 && ny < height as i16 {
                        board.empty_pos.push((nx as u8, ny as u8));
                    }
                }
            }
            if let Some(region) = board.regions.iter().find(|r| r.contains(&selected_pos)) {
                for &p in region {
                    if p != selected_pos {
                        board.empty_pos.push(p);
                    }
                }
            }
        }
    }
}
