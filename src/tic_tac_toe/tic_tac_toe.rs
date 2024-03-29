use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerPiece {
    Empty,
    X,
    O,
}

impl fmt::Display for PlayerPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerPiece::Empty => write!(f, "-"),
            PlayerPiece::X => write!(f, "X"),
            PlayerPiece::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

pub fn player_piece_to_player(player: &Player) -> PlayerPiece {
    match player {
        Player::X => PlayerPiece::X,
        Player::O => PlayerPiece::O,
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        player_piece_to_player(&self).fmt(f)
    }
}
