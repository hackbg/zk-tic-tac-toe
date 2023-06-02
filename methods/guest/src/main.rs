#![no_main]

use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
    serde::to_vec
};
use game::{TicTacToe, Point};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let mut game: TicTacToe = env::read();
    let point: Point = env::read();

    let result = game.make_move(point).unwrap();
}
