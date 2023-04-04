use std::fmt;
use std::io;
use std::io::Write;

use colored::ColoredString;
use colored::Colorize;

use crate::pawn::Pawn;

const ROWS: usize = 6;
const COLS: usize = 7;

pub struct ConnectFour {
    /// Board matrix, stores the colored emojis.
    board: [[Pawn; COLS]; ROWS],
    turn: Pawn,
    is_connected: bool,
    is_draw: bool,
}

impl ConnectFour {
    /// Check if the last placed pawn is connected to four other pawns of the same color.
    /// Optimized to only check around the last placed pawn instead of the whole board.
    /// Note: Should be called after placing the pawn and before switching the pawn.
    // fn is_four_connected(self: &Self, row: usize, col: usize) -> bool {}

    fn is_full(self: &Self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&item| item != Pawn::White))
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
        // self.is_connected = self.is_four_connected(row, col);
        self.is_draw = self.is_full();
        self.turn.switch();
    }

    #[inline]
    fn input_col(buffer: &str) -> usize {
        let mut input = String::new();
        io::stdout().write(buffer.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }

    fn get_empty_spot(self: &Self, col: usize) -> Option<usize> {
        for row in (0..ROWS).rev() {
            if !self.is_set(row, col) {
                return Some(row);
            }
        }
        None
    }

    pub fn run() {
        let mut game = Self::new();
        let mut col;

        loop {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("{}", game);

            // if game.is_connected {
            //     println!("{} won!", game.turn);
            //     break;
            // } else if game.is_draw {
            //     println!("Draw!");
            //     break;
            // }

            println!("{}'s turn", game.turn);

            col = Self::input_col("Enter column: ");
            if let Some(row) = game.get_empty_spot(col) {
                game.place(row, col);
            } else {
                println!("Column is full!");
            }
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
        use BoxTextures::*;
        let texture = match self {
            BottomLeftCorner => "└",
            BottomRightCorner => "┘",
            VerticalBar => "│",
            HorizontalBar => "─",
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
                    .repeat(COLS * 2)
                    .as_str()
                + BoxTextures::BottomRightCorner.to_string().as_str(),
        ]
        .join("\n")
        .fmt(f)?;
        Ok(())
    }
}
