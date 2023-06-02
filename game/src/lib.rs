use serde::{Serialize, Deserialize};

const CELL_COUNT: usize = 3;

#[derive(Serialize, Deserialize)]
pub struct TicTacToe {
    board: [[Cell; CELL_COUNT]; CELL_COUNT],
    previous: Player,
    state: State
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Player {
    A,
    B
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct Point {
    x: usize,
    y: usize
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum MoveError {
    PointOutOfBounds,
    CellOccupied,
    GameFinished
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum State {
    InProgress,
    Stalemate,
    Winner(Player)
}

// Keeping this enum without payloads so that its size is a single byte.
#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
enum Cell {
    Player1,
    Player2,
    Vacant
}

impl TicTacToe {
    pub fn new() -> Self {
        let board = [
            [Cell::Vacant; CELL_COUNT],
            [Cell::Vacant; CELL_COUNT],
            [Cell::Vacant; CELL_COUNT]
        ];

        Self {
            board,
            previous: Player::B,
            state: State::InProgress
        }
    }

    pub fn make_move(&mut self, point: Point) -> Result<(), MoveError> {
        if self.state != State::InProgress {
            return Err(MoveError::GameFinished);
        }

        if point.x >= CELL_COUNT || point.y >= CELL_COUNT {
            return Err(MoveError::PointOutOfBounds);
        }

        let ref mut cell = self.board[point.y][point.x];
        if *cell != Cell::Vacant {
            return Err(MoveError::CellOccupied);
        }

        let current = self.previous.flip();

        self.previous = current;
        *cell = current.into();

        self.update_state();

        Ok(())
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn current_player(&self) -> Player {
        self.previous.flip()
    }

    fn update_state(&mut self) {
        let mut has_vacant = false;

        let mut left_diag = self.board[0][0] != Cell::Vacant;
        let mut right_diag = self.board[0][CELL_COUNT - 1] != Cell::Vacant;

        let mut winner: Option<Cell> = None;

        for y in 0..CELL_COUNT {
            let mut horizontal = self.board[y][0] != Cell::Vacant;
            let mut vertical = self.board[0][y] != Cell::Vacant;

            if left_diag && y > 0 {
                left_diag = self.board[y][y] == self.board[y - 1][y - 1];
            }

            if right_diag && y > 0 {
                let last_index = CELL_COUNT - 1;
                
                right_diag = self.board[y][last_index - y] ==
                    self.board[y - 1][last_index - y + 1];
            }

            for x in 0..CELL_COUNT {
                let cell = self.board[y][x];

                if cell == Cell::Vacant {
                    has_vacant = true;
                }

                if horizontal && x > 0 {
                    horizontal = cell == self.board[y][x - 1];
                }

                if vertical && x > 0 {
                    vertical = self.board[x][y] == self.board[x - 1][y];
                }
            }

            if horizontal {
                winner = Some(self.board[y][0]);

                break;
            }

            if vertical {
                winner = Some(self.board[0][y]);

                break;
            }
        }

        if left_diag {
            winner = Some(self.board[0][0]);
        }

        if right_diag {
            winner = Some(self.board[0][CELL_COUNT - 1]);
        }

        if let Some(winner) = winner {
            let player = match winner {
                Cell::Player1 => Player::A,
                Cell::Player2 => Player::B,
                Cell::Vacant => unreachable!()
            };

            self.state = State::Winner(player);
        } else if !has_vacant {
            self.state = State::Stalemate;
        }
    }

    pub fn print_board(&self) {
        let mut row = [0u8; CELL_COUNT * 2];

        for y in 0..CELL_COUNT {
            let mut i = 0;
            
            for x in 0..CELL_COUNT {
                let cell = match self.board[y][x] {
                    Cell::Player1 => 'X',
                    Cell::Player2 => 'O',
                    Cell::Vacant => ' '
                } as u8;

                row[i] = cell;
                row[i + 1] = '|' as u8;
                i += 2;
            }

            println!(
                "|{}",
                unsafe { std::str::from_utf8_unchecked(&row) }
            );
        }
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Into<Cell> for Player {
    fn into(self) -> Cell {
        match self {
            Self::A => Cell::Player1,
            Self::B => Cell::Player2
        }
    }
}

impl Player {
    pub fn flip(&self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A
        }
    }
}
