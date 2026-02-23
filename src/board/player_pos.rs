use crate::app::App;

impl App {
    pub fn change_position(&mut self, x: i8, y: i8) {
        let pos = &mut self.selected_pos;
        let board = &mut self.board;
        if x > 0 {
            if pos.0 == board.size.0 - 1 {
                pos.0 = 0;
            } else {
                pos.0 += 1;
            }
        } else if x < 0 {
            if pos.0 != 0 {
                pos.0 -= 1;
            } else {
                pos.0 = board.size.0 - 1;
            }
        }

        if y > 0 {
            if pos.1 != 0 {
                pos.1 -= 1;
            } else {
                pos.1 = board.size.1 - 1;
            }
        } else if y < 0 {
            if pos.1 != board.size.1 - 1 {
                pos.1 += 1;
            } else {
                pos.1 = 0;
            }
        }
    }
}
