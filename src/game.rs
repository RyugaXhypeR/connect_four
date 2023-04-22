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

    fn get_empty_spot(self: &Self, col: usize) -> Option<usize> {
        (0..MAX_ROW).rev().find(|&row| !self.is_set(row, col))
    }

    /// Check if the last placed pawn is connected to four other pawns of the same color.
    /// Optimized to only check around the last placed pawn instead of the whole board.
    /// Note: Should be called after placing the pawn and before switching the pawn.
    fn is_four_connected(self: &Self, row: usize, col: usize) -> bool {
        [
            // Horizontal check
            self.board[row].try_into().unwrap(),
            // Vertrical check
            self.board.iter().map(|r| r[col]).collect::<Vec<Pawn>>(),
            // Diagonal (Bottom left to top right)
            (0..ROWS)
                .rev()
                .map(|i| (0..COLS).map(|j| self.board[i][j]).collect::<Vec<Pawn>>())
                .flatten()
                .collect::<Vec<Pawn>>(),
            // Diagonal (Top left to bottom right)
            (0..ROWS)
                .rev()
                .map(|i| {
                    (1..COLS)
                        .rev()
                        .map(|j| self.board[i][j])
                        .collect::<Vec<Pawn>>()
                })
                .flatten()
                .collect::<Vec<Pawn>>(),
        ]
        .iter()
        .map(|arr| arr.iter().filter(|&item| *item == self.turn).count())
        .any(|count| count >= 4)
    }

    fn is_full(self: &Self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&item| item != Pawn::White))
    }
    fn is_over(self: &Self) -> bool {
        self.is_connected || self.is_draw
    }

    fn is_set(self: &Self, row: usize, col: usize) -> bool {
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
        io::stdout().write(buffer.as_bytes()).unwrap();
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

    fn render_board(self: &Self) {
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
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
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

    #[test]
    fn test_connected_vertical() {
        let mut game = ConnectFour::new();
        game.place(0, 0);
        game.place(1, 0);
        game.place(2, 0);
        assert!(!game.is_connected);
        game.place(3, 0);
        println!("Vertial: {}", game);
        assert!(game.is_connected);
    }

    #[test]
    fn test_connected_horizontal() {
        let mut game = ConnectFour::new();
        game.place(0, 0);
        game.place(0, 1);
        game.place(0, 2);
        assert!(!game.is_connected);
        game.place(0, 3);
        println!("Horizontal: {}", game);
        assert!(game.is_connected);
    }

    #[test]
    fn test_connected_diagonal_bottom_left_to_top_right() {
        let mut game = ConnectFour::new();
        game.place(3, 0);
        game.place(2, 1);
        game.place(1, 2);
        assert!(!game.is_connected);
        game.place(0, 3);
        println!("Botton to Top: {}", game);
        assert!(game.is_connected);
    }

    #[test]
    fn test_connected_diagonal_top_left_to_bottom_right() {
        let mut game = ConnectFour::new();
        game.place(3, 3);
        game.place(2, 2);
        game.place(1, 1);
        assert!(!game.is_connected);
        game.place(0, 0);
        println!("Top to Bottom: {}", game);
        assert!(game.is_connected);
    }
}
