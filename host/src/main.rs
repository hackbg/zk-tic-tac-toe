use std::io::{self, Write};

use methods::{MAKE_MOVE_ELF, MAKE_MOVE_ID};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    sha::{Sha256, Impl, Digest},
    Executor, ExecutorEnv, SessionReceipt
};
use game::{TicTacToe, State, Player, Point, VmResponse};

struct Server {
    game: TicTacToe
}

#[derive(Debug)]
struct Client {
    game_state: State,
    state_hash: Digest
}

fn main() {
    println!("
Tic-Tac-Toe using the Risc0 VM.\n
On each turn the current player has to input the coordinates \
of the cell they want to fill in the form of \"x y\" where \"0 0\" \
points to the top leftmost cell. For example: if the player wants \
to fill the cell in the middle, they must provide the following input: \"1 1\".
    ");
    
    let mut server = Server::new();

    let mut player_a = Client::new();
    let mut player_b = Client::new();

    while let State::InProgress = server.game.state() {
        server.game.print_board();

        match server.game.current_player() {
            Player::A => print!("Player 1 turn: "),
            Player::B => print!("Player 2 turn: "),
        };

        io::stdout().flush().unwrap();

        let point = Server::wait_for_input();
        let receipt = server.execute_move(point);

        player_a.verify_receipt(&receipt);
        player_b.verify_receipt(&receipt);

        let resp: VmResponse = from_slice(&receipt.journal).unwrap();
        server.game = resp.game;
    }

    match server.game.state() {
        State::Stalemate => println!("Stalemate!"),
        State::Winner(Player::A) => println!("Player 1 wins!"),
        State::Winner(Player::B) => println!("Player 2 wins!"),
        State::InProgress => unreachable!()
    }

    player_a.on_game_ended();
    player_b.on_game_ended();
}

impl Server {
    pub fn new() -> Self {
        Self {
            game: TicTacToe::new()
        }
    }

    pub fn execute_move(&self, point: Point) -> SessionReceipt {
        let env = ExecutorEnv::builder()
            .add_input(&to_vec(&self.game).unwrap())
            .add_input(&to_vec(&point).unwrap())
            .build();

        let mut executor = Executor::from_elf(env, MAKE_MOVE_ELF).unwrap();
        let session = executor.run().unwrap();

        session.prove().unwrap()
    }

    pub fn wait_for_input() -> Point {
        let stdin = io::stdin();
        let mut line = String::with_capacity(4);

        loop {
            stdin.read_line(&mut line).unwrap();
            
            let line_trimmed = line.trim_end();
            let bytes = line_trimmed.as_bytes();

            if bytes.len() == 3 && bytes[1] == ' ' as u8 &&
                is_ascii_num(bytes[0]) && is_ascii_num(bytes[2])
            {
                let x = line_trimmed[0..1].parse().unwrap();
                let y = line_trimmed[2..3].parse().unwrap();

                return Point::new(x, y);
            }

            println!("Bad input. Try again...");
            line.clear();
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            state_hash: TicTacToe::initial_hash(),
            game_state: State::InProgress
        }
    }

    pub fn verify_receipt(&mut self, receipt: &SessionReceipt) {
        assert_eq!(self.game_state, State::InProgress, "Game has already ended!");

        receipt.verify(MAKE_MOVE_ID)
            .expect("receipt verification failed");

        let resp: VmResponse = from_slice(&receipt.journal).unwrap();
        assert_eq!(self.state_hash, resp.prev_state_hash, "Game state hash mismatch!");

        self.game_state = resp.game.state();
        self.state_hash = *Impl::hash_bytes(&resp.game.as_bytes());
    }

    pub fn on_game_ended(self) {
        assert_ne!(
            self.game_state,
            State::InProgress,
            "Server signaled that the game has ended but the client state does not reflect that!"
        );
    }
}

fn is_ascii_num(byte: u8) -> bool {
    byte >= 48 && byte <= 57
}
