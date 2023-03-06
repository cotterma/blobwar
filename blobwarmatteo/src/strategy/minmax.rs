//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let depth:u8 = self.0;
        let player:bool=state.current_player;
        return state.movements().par_bridge().max_by_key(|movement| minmax_recursif(&state.play(movement), depth-1));
    }   
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

fn minmax_recursif(state: &Configuration, depth: u8) -> i8{
    if depth==0{
        return state.value();
    }
    else if state.movements().peekable().peek().is_none(){
        return -minmax_recursif(&state.skip_play(), depth-1);
    }
    return -state.movements().par_bridge().map(|movement| minmax_recursif(&state.play(&movement), depth-1)).max().unwrap();
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
