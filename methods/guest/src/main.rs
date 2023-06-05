#![no_main]

use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256}
};
use game::{VmResponse, TicTacToe, Point};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let mut game: TicTacToe = env::read();
    let point: Point = env::read();

    let prev_state_hash = *Impl::hash_bytes(&game.as_bytes());

    game.make_move(point).unwrap();

    env::commit(&VmResponse {
        game,
        prev_state_hash
    });
}
