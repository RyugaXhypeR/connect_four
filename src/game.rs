use crate::pawn::Pawn;

const ROWS: usize = 6;
const COLS: usize = 7;

struct ConnectFour {
    /// Board matrix, stores the colored emojis.
    board: [[Pawn; COLS]; ROWS],
    turn: Pawn,
    is_connected: bool,
    is_draw: bool,
}

impl ConnectFour {
    fn is_four_connected(self: &Self, row: usize, col: usize) -> bool {
        if col + 4 < COLS {
            for i in 0..4 {
                if self.board[row][col + i] != self.turn {
                    return false;
                }
            }
        }

        if col - 4 >= 0 {
            for i in 0..4 {
                if self.board[row][col - i] != self.turn {
                    return false;
                }
            }
        }

        if row + 4 < ROWS {
            for i in 0..4 {
                if self.board[row + i][col] != self.turn {
                    return false;
                }
            }
        }

        if row - 4 >= 0 {
            for i in 0..4 {
                if self.board[row - i][col] != self.turn {
                    return false;
                }
            }
        }

        if row + 4 < ROWS && col + 4 < COLS {
            for i in 0..4 {
                if self.board[row + i][col + i] != self.turn {
                    return false;
                }
            }
        }

        if row - 4 >= 0 && col - 4 >= 0 {
            for i in 0..4 {
                if self.board[row - i][col - i] != self.turn {
                    return false;
                }
            }
        }

        true
    }

    fn is_full(self: &Self) -> bool {
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.board[row][col] == Pawn::White {
                    return false;
                }
            }
        }
        true
    }

    fn is_set(self: &Self, row: usize, col: usize) -> bool {
        self.board[row][col] != Pawn::White
    }
}

impl ConnectFour {
    fn new() -> Self {
        Self {
            board: [[Pawn::White; COLS]; ROWS],
            // Red starts first.
            turn: Pawn::Red,
            is_connected: false,
            is_draw: false,
        }
    }

    fn place(&mut self, row: usize, col: usize) {
        self.board[row][col] = self.turn;
        self.is_connected = self.is_four_connected(row, col);

        if self.is_connected || self.is_full() {
            self.is_draw = true;
        }

        self.turn.switch();
    }
}

impl fmt::Display for ConnectFour {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.board.iter() {
            for pawn in row.iter() {
                write!(f, "{}", pawn)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
