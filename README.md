# Risc0 Tic-Tac-Toe

The classic tic-tac-toe game, implemented in Rust, that leverages the [Ric0](https://www.risc0.com)
zero-knowledge virtual machine. It uses a mocked client-server architecture where the clients are
the two players and the server keeps the game state and takes inputs from the players that it passes
to the VM to execute the game logic. It then sends the execution receipts to the players for them to
verify. The players keep a hash of the previous game state that they compare to the one provided by
the output of the Risc0 receipt.

## Project structure

 - `game` crate - defines the tic-tac-toe state and implements the game logic
 - `methods` crate - defines the method that is being executed inside the Risc0 VM
 - `host` crate - the executable that brings it all together. Implements the game loop, running the
 VM based on player input, generating the execution proof and sending to the players to verify.
