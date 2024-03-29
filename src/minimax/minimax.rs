use crate::tic_tac_toe::{Player, PlayerPiece, TicTacToe};

pub struct Children<'a> {
    tic_tac_toe: &'a TicTacToe,
    next_index: usize,
}

pub fn generate_children(tic_tac_toe: &TicTacToe) -> Children {
    Children {
        tic_tac_toe,
        next_index: 0,
    }
}

impl Iterator for Children<'_> {
    type Item = (TicTacToe, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.next_index..9 {
            if self.tic_tac_toe.get_board()[i] == PlayerPiece::Empty {
                let mut new_game = self.tic_tac_toe.get_board().clone();
                let player_piece = match self.tic_tac_toe.get_current_player() {
                    Player::X => PlayerPiece::X,
                    Player::O => PlayerPiece::O,
                };
                new_game[i] = player_piece;

                let child =
                    TicTacToe::new_with_board(new_game, self.tic_tac_toe.get_other_player());
                self.next_index = i + 1;
                return Some((child, i));
            }
        }
        None
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_generate_children() {
        let game = TicTacToe::new();
        let children = generate_children(&game).collect::<Vec<_>>();
        assert_eq!(children.len(), 9);
    }
}
