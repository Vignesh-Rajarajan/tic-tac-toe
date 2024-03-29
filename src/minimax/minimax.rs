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
                let mut new_game = *self.tic_tac_toe.get_board();
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

    #[test]
    fn test_generate_children_with_one_move() {
        let game = TicTacToe::new();
        let mut children = generate_children(&game);

        let (game, index) = children.next().unwrap();
        assert_eq!(index, 0);
        assert_eq!(game.get_board()[0], PlayerPiece::X);

        let (game, index) = children.next().unwrap();
        assert_eq!(index, 1);
        assert_eq!(game.get_board()[1], PlayerPiece::X);
    }

    #[test]
    fn test_generate_children_with_two_moves() {
        let game = TicTacToe::new_with_board(
            [
                PlayerPiece::X,
                PlayerPiece::O,
                PlayerPiece::O,
                PlayerPiece::Empty,
                PlayerPiece::X,
                PlayerPiece::X,
                PlayerPiece::Empty,
                PlayerPiece::X,
                PlayerPiece::O,
            ],
            Player::O,
        );

        let mut children = generate_children(&game);
        let (game, index) = children.next().unwrap();
        assert_eq!(index, 3);
        assert_eq!(game.get_board()[3], PlayerPiece::O);
    }
}
