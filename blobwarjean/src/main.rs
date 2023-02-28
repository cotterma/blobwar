extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{MinMax};
use std::time::Instant;



fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    let start = Instant::now();
    game.battle(MinMax(3), MinMax(2));
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
