//! Dumb greedy algorithm.
use libc::INT_MAX;
use nix::sys::stat::stat;

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use std::{fmt, ptr::null};

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // let mut movement:Option<Movement>;
        // let mut max:i8=-INT_MAX as i8;
        // for m in state.movements(){
        //     let new_configuration:Configuration = state.clone();
        //     let owned_movement = m.clone();
        //     new_configuration.play(&owned_movement);
        //     let value:i8=new_configuration.value();
        //     if(value>max){
        //         movement=Some(owned_movement);
        //         max=value;
        //     } 
        // }
        // println!("greedy {}", state.current_player);
        return state.movements().max_by_key(|movement| state.play(movement).value());
    }
}