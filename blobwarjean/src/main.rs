extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{MinMax, AlphaBeta};
use blobwar::strategy::Strategy;
use blobwar::strategy::alpha_beta_anytime;
use std::time::Instant;



fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    let start = Instant::now();
    alpha_beta_anytime(&game);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
