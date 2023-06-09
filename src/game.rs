use std::fmt;
use std::io;
use std::io::Write;

use colored::Colorize;

use crate::pawn::Pawn;

const MAX_ROW: usize = 6;
const MAX_COL: usize = 7;
const MIN_CONNECT: usize = 4;

pub struct ConnectFour {
    /// Board matrix, stores the colored emojis.
    board: [[Pawn; MAX_COL]; MAX_ROW],
    turn: Pawn,
    is_connected: bool,
    is_draw: bool,
    moves_stack: Vec<(Pawn, (usize, usize))>,
}

/// Controller for `ConnectFour`, handles the concept / logic of the game.
impl ConnectFour {
    fn new() -> Self {
        Self {
            board: [[Pawn::White; MAX_COL]; MAX_ROW],
            // Red starts first.
            turn: Pawn::Red,
            is_connected: false,
            is_draw: false,
            moves_stack: Vec::new(),
        }
    }

    /// Get empty row from the respective column.
    ///
    /// # Panics
    /// When `col` is not in range `[0, MAX_COL)`
    fn get_empty_spot(&self, col: usize) -> Option<usize> {
        assert!(col < MAX_COL);
        (0..MAX_ROW).rev().find(|&row| !self.is_set(row, col))
    }

    /// Check if the last placed pawn is connected to four other pawns of the same color.
    /// Optimized to only check around the last placed pawn instead of the whole board.
    ///
    /// **Note:** Should be called after placing the pawn and before switching the pawn.
    ///
    /// # Panics
    /// When `col` is not in range `[0, MAX_COL)`
    /// When `row` is not in range `[0, MAX_ROW)`
    fn is_four_connected(&self, row: usize, col: usize) -> bool {
        assert!(col < MAX_COL);
        assert!(row < MAX_ROW);

        // Checking if either of the axis from the pivot index (row, col) have any connections.
        // Doing both at the same time, shouldn't be very expensive.
        //
        // Basically, just constructing two arrays: one where the indices appear in the horizontal
        // axis and the other in which the indices appear in the vertical axis.
        // Then we get a window of `MIN_CONNECT` elements and check if they all are equal to
        // `self.turn`.
        let axis_checks = [
            // Horizontal check
            self.board[row].try_into().unwrap(),
            // Vertical check
            self.board.iter().map(|r| r[col]).collect::<Vec<Pawn>>(),
        ]
        .iter()
        .map(|r| {
            r.windows(MIN_CONNECT)
                .any(|window| window.iter().all(|&item| item == self.turn))
        })
        .any(|connected| connected);

        if axis_checks {
            return true;
        }

        [
            // Diagonal (Top left to bottom right)
            self.board
                .iter()
                .enumerate()
                .flat_map(|(i, r)| {
                    r.iter()
                        .enumerate()
                        .filter(move |(j, _)| {
                            row as isize - i as isize == col as isize - *j as isize
                        })
                        .map(|(_, &p)| p)
                })
                .collect::<Vec<Pawn>>(),
            self.board
                .iter()
                .enumerate()
                .flat_map(|(i, r)| {
                    r.iter()
                        .enumerate()
                        .filter(move |(j, _)| {
                            row as isize - i as isize == *j as isize - col as isize
                        })
                        .map(|(_, &p)| p)
                })
                .collect::<Vec<Pawn>>(),
        ]
        .iter()
        .map(|r| {
            r.windows(MIN_CONNECT)
                .any(|window| window.iter().all(|&item| item == self.turn))
        })
        .any(|connected| connected)
    }

    fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&item| item != Pawn::White))
    }

    fn is_over(&self) -> bool {
        self.is_connected || self.is_draw
    }

    fn is_set(&self, row: usize, col: usize) -> bool {
        self.board[row][col] != Pawn::White
    }

    fn place(&mut self, row: usize, col: usize) {
        self.moves_stack.push((self.turn, (row, col)));
        self.board[row][col] = self.turn;
        self.is_connected = self.is_four_connected(row, col);
        self.is_draw = self.is_full();
    }
}

/// View for `ConnectFour`, handles the io of the game.
impl ConnectFour {
    /// Helper function which prints the buffer and takes the column number as input.
    /// Also converts the column number to `usize`
    #[inline]
    fn input_column_number(buffer: &str) -> usize {
        let mut input = String::new();
        print!("{buffer}");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }

    #[inline]
    fn validate_column_number(col: usize) -> Result<usize, &'static str> {
        if col > MAX_COL {
            return Err("Column number is out of bounds!");
        }
        Ok(col)
    }

    fn render_board(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", self);
    }

    pub fn run() {
        let mut game = Self::new();
        let mut col: usize;

        while !game.is_over() {
            // Clear the terminal and place the cursor at the beginning.
            game.render_board();
            println!("{}'s turn", game.turn);

            col = match Self::validate_column_number(Self::input_column_number(
                "Enter column number: ",
            )) {
                Ok(col) => col,
                Err(_) => continue,
            };
            game.place(game.get_empty_spot(col).unwrap(), col);
            game.turn.switch();
        }

        game.render_board();
        if game.is_connected {
            println!("{} won!", game.moves_stack.last().unwrap().0);
        } else {
            println!("Draw!");
        }
    }
}

enum BoxTextures {
    BottomLeftCorner,
    BottomRightCorner,
    VerticalBar,
    HorizontalBar,
}

impl fmt::Display for BoxTextures {
    /// Display the box textures using the colored crate.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let texture = match self {
            BoxTextures::BottomLeftCorner => "└",
            BoxTextures::BottomRightCorner => "┘",
            BoxTextures::VerticalBar => "│",
            BoxTextures::HorizontalBar => "─",
        };

        write!(f, "{}", texture.yellow().bold())
    }
}

impl fmt::Display for ConnectFour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        [
            // Open top part of the board.
            // Part from which the pawn will fall.
            "\n".to_string(),
            // The game board formatted with vertical bars surrounding it.
            self.board
                .iter()
                .map(|row| {
                    BoxTextures::VerticalBar.to_string()
                        + &row
                            .iter()
                            .map(|pawn| pawn.to_string())
                            .collect::<Vec<String>>()
                            .join("")
                        + BoxTextures::VerticalBar.to_string().as_str()
                })
                .collect::<Vec<String>>()
                .join("\n"),
            // Bottom part of the board.
            BoxTextures::BottomLeftCorner.to_string()
                + BoxTextures::HorizontalBar
                    .to_string()
                    .repeat(MAX_COL * 2)
                    .as_str()
                + BoxTextures::BottomRightCorner.to_string().as_str(),
        ]
        .join("\n")
        .fmt(f)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectFour;
    use super::Pawn::{self, *};

    fn from_board(board: [[Pawn; 7]; 6]) -> ConnectFour {
        ConnectFour {
            board,
            is_draw: false,
            is_connected: false,
            turn: Red,
            moves_stack: vec![],
        }
    }

    #[test]
    fn test_connected_vertical_pass() {
        let connect_four = from_board([
            [White; 7],
            [White; 7],
            [Red, White, White, White, White, White, White],
            [Red, White, White, White, White, White, White],
            [Red, White, White, White, White, White, White],
            [Red, White, White, White, White, White, White],
        ]);
        assert!(connect_four.is_four_connected(2, 0));
    }

    #[test]
    fn test_connected_vertical_fail() {
        let connect_four = from_board([
            [Red, White, White, White, White, White, White],
            [Red, White, White, White, White, White, White],
            [White, White, White, White, White, White, White],
            [Red, White, White, White, White, White, White],
            [Red, White, White, White, White, White, White],
            [White; 7],
        ]);
        assert!(!connect_four.is_four_connected(4, 0));
    }

    #[test]
    fn test_connected_horizontal_pass() {
        let connect_four = from_board([
            [Red, Red, Red, Red, White, White, White],
            [White; 7],
            [White; 7],
            [White; 7],
            [White; 7],
            [White; 7],
        ]);
        assert!(connect_four.is_four_connected(0, 3));
    }

    #[test]
    fn test_connected_horizontal_fail() {
        let connect_four = from_board([
            [Red, Red, Red, White, Red, White, White],
            [White; 7],
            [White; 7],
            [White; 7],
            [White; 7],
            [White; 7],
        ]);
        assert!(!connect_four.is_four_connected(0, 4));
    }

    #[test]
    fn test_connected_diagonal_top_left_to_bottom_right_pass() {
        let connect_four = from_board([
            [White; 7],
            [White; 7],
            [Red, White, White, White, White, White, White],
            [White, Red, White, White, White, White, White],
            [White, White, Red, White, White, White, White],
            [White, White, White, Red, White, White, White],
        ]);
        assert!(connect_four.is_four_connected(2, 0));
    }

    #[test]
    fn test_connected_diagonal_top_left_to_bottom_right_fail() {
        let connect_four = from_board([
            [White; 7],
            [Red, White, White, White, White, White, White],
            [White, Red, White, White, White, White, White],
            [White, White, White, White, White, White, White],
            [White, White, White, Red, White, White, White],
            [White, White, White, White, Red, White, White],
        ]);
        assert!(!connect_four.is_four_connected(1, 0));
    }

    #[test]
    fn test_connected_diagonal_bottom_left_to_top_right_pass() {
        let connect_four = from_board([
            [White; 7],
            [White; 7],
            [White, White, White, Red, White, White, White],
            [White, White, Red, White, White, White, White],
            [White, Red, White, White, White, White, White],
            [Red, White, White, White, White, White, White],
        ]);
        assert!(connect_four.is_four_connected(5, 0));
    }

    #[test]
    fn test_connected_diagonal_bottom_left_to_top_right_fail() {
        let connect_four = from_board([
            [White; 7],
            [White, White, White, White, Red, White, White],
            [White, White, White, Red, White, White, White],
            [White, White, White, White, White, White, White],
            [White, Red, White, White, White, White, White],
            [Red, White, White, White, White, White, White],
        ]);
        assert!(!connect_four.is_four_connected(5, 0));
    }
}
