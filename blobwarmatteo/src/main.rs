extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{Greedy, Human, MinMax, AlphaBeta};
extern crate gnuplot;
use gnuplot::{Caption, Color, Figure, AxesCommon};


fn main(){
    // let n = 10;
    // let mut average_on_n:f64 = 0.0;
    // for i in 0..n{
        let board = Default::default();
        let mut game = Configuration::new(&board);
        let (times, perfs) = game.battle(AlphaBeta(5), AlphaBeta(5));
        // println!("{:?}", times);
        // println!("{:?}", perfs);
        // let average = perfs.iter().fold(0 as f64, |acc, x| acc+x) / times.last().unwrap().to_owned() as f64;
        // average_on_n = average_on_n + average;
        // println!("Average time taken for player_one to find a movement in this game : {}", average);
        let mut fg = Figure::new();
        fg.axes2d()
            .points(&times, &perfs, &[Caption("Computation time"), Color("blue")]);
        fg.set_title("AlphaBetaPar(5) fighting each others, computation time(move number)");
        fg.show().unwrap();
    // }
    // average_on_n = average_on_n / n as f64;
    // println!("Average time taken for player_one to find a movement in {} games : {}", n, average_on_n);
}
