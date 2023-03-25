use std::process::exit;

use getch::Getch;

use crate::board::{Board};

enum Action {
    MoveRight,
    MoveDown,
    MoveLeft,
    MoveUp,
    Enter,
    Exit,
    Invalid
}

enum GameState {
    WaitingForMarkerSelection,
    WaitingForMoveTarget
}

#[derive(Clone, Copy)]
struct CellPosition {
    r: usize,
    c: usize
}

pub struct Game {
    board: Board,
    cursor: CellPosition,
    preparedCell: Option<CellPosition>,
    state: GameState
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::new();
        let r = board.get_n_row() - 1;
        let c = 0 as usize;

        board.focus(r, c);
        board.draw();

        Game{board, cursor: CellPosition { r, c }, preparedCell: None, state: GameState::WaitingForMarkerSelection}
    }

    fn get_action(&self) -> Action {
        let b = Getch::new().getch().expect("error while getting input");

        match b {
            13 => Action::Enter,
            75 => Action::MoveLeft,
            72 => Action::MoveUp,
            77 => Action::MoveRight,
            80 => Action::MoveDown,
            27 => Action::Exit,
            _ => Action::Invalid
        }
    }

    pub fn next_turn(&mut self){

        let n_col = self.board.get_n_col();
        let n_row = self.board.get_n_row();

        let old_cursor = &self.cursor;
        let old_r = old_cursor.r;
        let old_c = old_cursor.c;
        let mut new_r: usize = old_cursor.r;
        let mut new_c: usize = old_cursor.c;
        let mut is_enter = false;

        match self.get_action() {
            Action::MoveUp => {
                if old_r > 0 {
                    new_r = old_r - 1;
                }
            },
            Action::MoveRight => {
                if old_c < n_col - 1 {
                    new_c = old_c + 1;
                }
            },
            Action::MoveDown => {
                if old_r < n_row - 1 {
                    new_r = old_r + 1;
                }
            },
            Action::MoveLeft => {
                if old_c > 0 {
                    new_c = old_c - 1;
                }
            },
            Action::Enter => {
                is_enter = true;
            },
            Action::Exit => {
                exit(0);
            },
            Action::Invalid => ()
        }

        let new_cursor = CellPosition{r: new_r, c: new_c};
        
        if is_enter {
            match self.state {
                GameState::WaitingForMarkerSelection => {
                    if self.board.is_cell_has_marker(new_r, new_c) {
                        self.preparedCell = Some(new_cursor);
                        self.state = GameState::WaitingForMoveTarget;
                    }
                },
                GameState::WaitingForMoveTarget => {
                    if self.board.is_cell_filled(new_r, new_c) && !self.board.is_cell_has_marker(new_r, new_c) {
                        self.board.move_cell(old_r, old_c, new_r, new_c);
                        self.preparedCell = None;
                        self.state = GameState::WaitingForMarkerSelection;
                    }
                }
            }
        }
        
        self.board.unfocus(old_cursor.r, old_cursor.c);
        match &self.preparedCell {
            Some(pos) => {
                self.board.prepare_to_move(pos.r, pos.c);
            },
            None => ()
        }
        self.board.focus(new_cursor.r, new_cursor.c);

        self.cursor = new_cursor;
        self.board.draw();

    }
}