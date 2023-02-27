//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        return state.movements().max_by_key(|movement: &Movement| nega_max(self.0-1, &state.play(movement)));
    }
}

fn nega_max(depth : u8, state : &Configuration) -> i8 {
    if depth == 0 || state.movements().peekable().peek().is_none(){/*gerer fin de partie*/
        return state.value();
    }
    else{
        let mut best_value = -127;
        let mut value;
        for movement in state.movements() {
            value = nega_max(depth - 1, &state.play(&movement));
            if best_value < value {
                best_value = value;
            }
        }
        return -best_value;// si on gagne de 1, notre adversaire gagne de -1 aka il perd de 1
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
