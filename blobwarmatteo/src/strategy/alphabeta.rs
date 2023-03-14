//! Alpha - Beta algorithm.
use std::fmt;

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        return state.movements().max_by_key(|movement: &Movement| {
            let next_state:Configuration = state.play(movement);
            if next_state.game_over(){
                return 127;
            }
            nega_alpha_beta(self.0-1, &next_state, -127, 127)
    });
    }
}

fn nega_alpha_beta(depth: u8, state : &Configuration, mut alpha: i8, beta: i8) -> i8 {
    if depth == 0 || state.game_over(){
        return state.value();
    }
    else if state.movements().peekable().peek().is_none(){
        return -nega_alpha_beta(depth - 1, &state.skip_play(), -beta, -alpha);
    }
    return -state.movements().fold_while(-127, |mut best_value, movement| {
        let value = nega_alpha_beta(depth - 1, &state.play(&movement), -beta, -alpha);
        if best_value < value {
            best_value = value;
        }
        if best_value >= beta {
            Done(best_value)
        }
        else{
            if alpha < best_value {
                alpha = best_value;
            }
            Continue(best_value)
        }
    }).into_inner();
}
