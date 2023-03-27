//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use rayon::iter::ParallelBridge;
use rayon::prelude::{ParallelIterator, IntoParallelIterator};
use crossbeam::atomic::AtomicCell;
use rayon::slice::ParallelSlice;


/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        return state.movements().par_bridge().max_by_key(|movement: &Movement| {
            let next_state:Configuration = state.play(movement);
            if next_state.game_over(){
                return 127;
            }
            minmax_recursif(self.0-1, &next_state)
    });
    }   
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

fn minmax_recursif(depth: u8, state: &Configuration) -> i8{
    if depth==0 || state.game_over(){
        return state.value();
    }
    else if state.movements().peekable().peek().is_none(){
        return -minmax_recursif(depth-1, &state.skip_play());
    }
    return -state.movements().par_bridge().map(|movement| minmax_recursif(depth-1, &state.play(&movement))).max().unwrap();
}

fn bigpower(depth: u8, state : &Configuration) -> i8 {
    if depth == 0 || state.game_over(){
        return state.value();
    }
    else if state.movements().peekable().peek().is_none(){
        return -bigpower(depth - 1, &state.skip_play());
    }
    let table:Vec<Movement> = state.movements().collect();
    return -max_rec(&table, depth, state).unwrap();
}

fn max_rec(s:&[Movement], depth:u8, state:&Configuration)->Option<i8>{
    if s.len()<=2{
        s.iter().map(|movement| bigpower(depth-1, &state.play(movement))).max()
    } else{
        let b=(s.len() as f64).sqrt().ceil() as usize;
        let maxes=s.par_chunks(b).map(|s| max_rec(s, depth, state).unwrap()).collect::<Vec<_>>();
        max_ultra_par(&maxes, depth, state)
    }
}

fn est_max(s:&[i8], i: usize) -> bool{
    s.into_par_iter().all(|e| *e <= s[i])
}

fn max_ultra_par(s:&[i8], depth:u8, state:&Configuration)->Option<i8>{
    let c =AtomicCell::new(None);
    (0..s.len()).into_par_iter().for_each(|i|{
        if est_max(s,i){
            c.store(Some(s[i]))
        }
    });
    c.load()
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
