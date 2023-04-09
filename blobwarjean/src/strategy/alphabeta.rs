//! Alpha - Beta algorithm.
use std::fmt;
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;


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
        let mut bestmovement = None;
        let mut bestvalue = -127;
        let mut value;
        let mut alpha = -127;
        for movement in state.movements(){
            value = nega_alpha_beta(self.0-1, &state.play(&movement), -127, 127);
            //value = nega_alpha_beta_para(self.0-1, &state.play(&movement), -127, 127);
            if value > bestvalue {
                bestvalue = value;
                bestmovement = Some(movement);
                if bestvalue > alpha {
                    alpha = bestvalue;
                }
            }
        }
        return bestmovement;
    }
}

fn nega_alpha_beta(depth: u8, state : &Configuration, mut alpha: i8, beta: i8) -> i8 {
    if depth == 0{
        return state.value();
    }
    else if state.movements().peekable().peek().is_none(){
        return -nega_alpha_beta(depth-1, &state.skip_play(), -beta, -alpha);
    }
    else{
        let mut best_value = -127;
        let mut value;
        for movement in state.movements() {
            value = nega_alpha_beta(depth - 1, &state.play(&movement), -beta, -alpha);
            if best_value < value {
                best_value = value;
            }
            if best_value >= beta {
                return -best_value;
            }
            if alpha < best_value {
                alpha = best_value;
            }
        }
        return -best_value;// si on gagne de 1, notre adversaire gagne de -1 aka il perd de 1
    }
}
fn nega_alpha_beta_para(depth: u8, state : &Configuration, mut alpha: i8, beta: i8) -> i8 {
    if depth == 0{
        return state.value();
    }
    else if depth == 1 {
        return -state.movements().par_bridge().map(|movement| state.play(&movement).value()).max().unwrap();
    }
    else if state.movements().peekable().peek().is_none(){
        return -nega_alpha_beta(depth-1, &state.skip_play(), -beta, -alpha);
    }
    else{
        let mut best_value = -127;
        let mut value;
        for movement in state.movements() {
            value = nega_alpha_beta_para(depth - 1, &state.play(&movement), -beta, -alpha);
            if best_value < value {
                best_value = value;
            }
            if best_value >= beta {
                return -best_value;
            }
            if alpha < best_value {
                alpha = best_value;
            }
        }
        return -best_value;// si on gagne de 1, notre adversaire gagne de -1 aka il perd de 1
    }
}

