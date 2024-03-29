mod tic_tac_toe;

pub use tic_tac_toe::{Player, PlayerPiece};

use self::tic_tac_toe::player_piece_to_player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TicTacToe {
    board: [PlayerPiece; 9],
    current_player: Player,
}

impl TicTacToe {
    const WINNING_POSISTION: [[usize; 3]; 8] = [
        // Horizontal
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        // Vertical
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        // Diagonal
        [0, 4, 8],
        [2, 4, 6],
    ];
    pub fn new() -> Self {
        Self {
            board: [PlayerPiece::Empty; 9],
            current_player: Player::X,
        }
    }

    pub fn new_with_board(board: [PlayerPiece; 9], current_player: Player) -> Self {
        Self {
            board,
            current_player,
        }
    }

    pub fn get_current_player(&self) -> Player {
        self.current_player
    }
    pub fn get_other_player(&self) -> Player {
        match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
    pub fn flip_player(&mut self) {
        self.current_player = self.get_other_player();
    }
    pub fn get_board(&self) -> [PlayerPiece; 9] {
        self.board
    }

    fn print_row(&self, range: std::ops::Range<usize>) {
        for i in range {
            print!("| {} ", self.board[i]);
        }
        println!("|");
    }

    pub fn print_board(&self) {
        for row in 0..3 {
            println!(" --- --- --- ");
            self.print_row(row * 3..(row + 1) * 3);
        }
        println!(" --- --- --- ");
    }

    pub fn play_at(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= 9 || index < 0 {
            return Err("Index out of bounds");
        }
        if self.board[index] != PlayerPiece::Empty {
            return Err("Position already taken");
        }
        self.board[index] = player_piece_to_player(&self.current_player);
        self.flip_player();
        Ok(())
    }

    pub fn has_empty_cells(&self) -> bool {
        self.board.iter().any(|&x| x == PlayerPiece::Empty)
    }

    pub fn check_wins(&self) -> Option<&[usize; 3]> {
        let other_piece = player_piece_to_player(&self.get_other_player());

        Self::WINNING_POSISTION.iter().find(|line| {
            self.board[line[0]] == other_piece
                && self.board[line[1]] == other_piece
                && self.board[line[2]] == other_piece
        })
    }
}
