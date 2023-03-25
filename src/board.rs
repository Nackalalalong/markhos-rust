use colored::*;

#[derive(Copy, Clone)]
pub enum MarkerSymbol {
    O,
    X
}

#[derive(Copy, Clone)]
pub struct Marker {
    symbol: MarkerSymbol,
    is_ultra_instinct: bool
}

pub enum CellFocusState {
    Focus,
    Unfocus,
    PrepareToMove
}

pub struct Cell {
    marker: Option<Marker>,
    fill: bool,
    state: CellFocusState
}

pub struct Board {
    n_row: usize,
    n_col: usize,
    cells: Vec<Vec<Cell>>
}

impl Cell {
    fn render(&self) -> ColoredString {
        let mut s = "   ";
        match &self.marker {
            Some(marker) => {
                s = match marker.symbol {
                    MarkerSymbol::O => " O ",
                    MarkerSymbol::X => " X "
                }
            },
            None => ()
        }

        let mut cs = s.bold().black();

        match &self.state {
            CellFocusState::Focus => {
                cs = cs.on_bright_green();
            },
            CellFocusState::PrepareToMove => {
                cs = cs.on_bright_magenta();
            },
            CellFocusState::Unfocus => {
                if self.fill {
                    cs = cs.on_bright_cyan();
                }
            }
        }
        
        return cs
    }

    pub fn remove_marker(&mut self) -> Option<Marker> {
        let marker = self.marker;
        self.marker = None;

        return marker;
    }

    pub fn focus(&mut self) {
        self.state = CellFocusState::Focus;
    }

    pub fn unfocus(&mut self) {
        self.state = CellFocusState::Unfocus;
    }

    pub fn prepare_to_move(&mut self) {
        self.state = CellFocusState::PrepareToMove;
    }
}

impl Board {
    pub fn new() -> Self {
        let n_row = 8;
        let n_col = 8;
        let mut cells: Vec<Vec<Cell>> = Vec::new();

        for r in 0..n_row {
            let mut row: Vec<Cell> = Vec::new();
            for c in 0..n_col {
                let fill = ( r + c ) % 2 == 0;
                let mut marker: Option<Marker> = None;
                if r < 2 && fill {
                    marker = Some(Marker{symbol: MarkerSymbol::O, is_ultra_instinct: false});
                }
                else if r >= n_row - 2 && fill {
                    marker = Some(Marker{symbol: MarkerSymbol::X, is_ultra_instinct: false});
                }
                
                let cell = Cell{marker, fill, state: CellFocusState::Unfocus};
                row.push(cell);
            }
            cells.push(row);
        }

        return Board { n_row, n_col, cells};
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn draw(&mut self) {
        
        self.clear_screen();

        for row in &self.cells {
            for cell in row {
                print!("{}", cell.render());
            }
            println!();
        }
    }

    pub fn focus(&mut self, r: usize, c: usize) {
        self.cells[r][c].focus();
    }

    pub fn unfocus(&mut self, r: usize, c: usize) {
        self.cells[r][c].unfocus();
    }

    pub fn prepare_to_move(&mut self, r: usize, c: usize) {
        self.cells[r][c].prepare_to_move();
    }

    pub fn move_cell(&mut self, from_r: usize, from_c: usize, to_r: usize, to_c: usize) {
        let marker = self.cells[from_r][from_c].remove_marker();
        self.cells[to_r][to_c].marker = marker;
    }

    pub fn is_cell_filled(&self, r: usize, c: usize) -> bool {
        self.cells[r][c].fill
    }

    pub fn is_cell_has_marker(&self, r: usize, c: usize) -> bool {
        let marker = &self.cells[r][c].marker;
        match marker {
            Some(_) => true,
            None => false
        }
    }

    pub fn get_n_row(&self) -> usize {
        self.n_row
    }

    pub fn get_n_col(&self) -> usize {
        self.n_col
    }
}