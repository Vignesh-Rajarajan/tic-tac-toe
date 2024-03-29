use crate::tic_tac_toe::{Player, TicTacToe};
use rand::prelude::SliceRandom;
use std::cmp;
mod minimax;
pub struct Node {
    tic_tac_toe: TicTacToe,
}

impl Node {
    pub fn new(tic_tac_toe: TicTacToe) -> Self {
        Self { tic_tac_toe }
    }
}

pub struct Minimax {
    player: Player,
}

impl Minimax {
    pub fn new(player: Player) -> Self {
        Self { player }
    }

    pub fn get_player(&self) -> Player {
        self.player
    }

    fn static_analysis(tic_tac_toe: &TicTacToe) -> i32 {
        match tic_tac_toe.get_other_player() {
            Player::X => i32::MAX,
            Player::O => i32::MIN,
        }
    }

    fn minimax(node: &Node, mut alpha: i32, mut beta: i32, maximising_player: bool) -> i32 {
        if node.tic_tac_toe.check_wins().is_some() {
            return Self::static_analysis(&node.tic_tac_toe);
        }
        if !node.tic_tac_toe.has_empty_cells() {
            return 0;
        }

        let mut best_value = if maximising_player {
            i32::MIN
        } else {
            i32::MAX
        };

        for (child, _index) in minimax::generate_children(&node.tic_tac_toe) {
            let child_score = Self::minimax(&Node::new(child), alpha, beta, !maximising_player);
            if maximising_player {
                best_value = cmp::max(child_score, best_value);
                alpha = cmp::max(best_value, alpha);
                if beta <= alpha {
                    break;
                }
            } else {
                best_value = cmp::min(child_score, best_value);
                beta = cmp::min(best_value, beta);
                if best_value < alpha {
                    break;
                }
            }
        }
        best_value
    }

    fn get_node_score(node: &mut Node, maximising_player: bool) -> i32 {
        Self::minimax(node, i32::MIN, i32::MAX, maximising_player)
    }

    pub fn get_best_move(&self, tic_tac_toe: &TicTacToe) -> usize {
        let mut best_score: Option<i32> = None;
        let mut best_indices = Vec::new();

        let maximising_player = tic_tac_toe.get_current_player() == Player::X;

        for (child, index) in minimax::generate_children(tic_tac_toe) {
            let mut node = Node::new(child);
            let score = Self::get_node_score(&mut node, !maximising_player);

            match best_score {
                Some(best) => {
                    if maximising_player && score > best || !maximising_player && score < best {
                        best_score = Some(score);
                        best_indices.clear();
                        best_indices.push(index);
                    } else if score == best {
                        best_indices.push(index);
                    }
                }
                None => {
                    best_score = Some(score);
                    best_indices.push(index);
                }
            }
        }

        *best_indices
            .choose(&mut rand::thread_rng())
            .expect("Could not find a best move")
    }
}
