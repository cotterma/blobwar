//! Alpha - Beta algorithm.
use std::fmt;
use std::collections::HashMap;
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_transpo_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBetaTranspo(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBetaTranspo(pub u8);

impl fmt::Display for AlphaBetaTranspo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta - transpo (max level: {})", self.0)
    }
}

impl Strategy for AlphaBetaTranspo {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        return state.movements().par_bridge().max_by_key(|movement: &Movement| 
            nega_alpha_beta_transpo(self.0-1, &state.play(movement), &mut HashMap::<(u64, u64), i8>::new(), &mut HashMap::<(u64, u64), i8>::new(), -127, 127));
    }
}

fn nega_alpha_beta_transpo(depth: u8, state : &Configuration, tp_cp: &mut HashMap<(u64, u64), i8>, tp_op: &mut HashMap<(u64,u64), i8>,
    mut alpha: i8, beta: i8) -> i8 {
    if depth == 0{
        return state.value();
    }
    else if state.movements().peekable().peek().is_none(){
        return -nega_alpha_beta_transpo(depth-1, &state.skip_play(), tp_op, tp_cp, -beta, -alpha);
    }
    else{
        let mut best_value = -127;
        let mut value;
        let mut pos;
        for movement in state.movements() {
            pos = state.play(&movement);
            if tp_cp.contains_key(&pos.get_hash()){
                value = *(tp_cp.get(&pos.get_hash()).unwrap());
            }
            else{
                value = nega_alpha_beta_transpo(depth - 1, &pos, tp_op, tp_cp, -beta, -alpha);
                tp_cp.insert(pos.get_hash(), value);
            }
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

