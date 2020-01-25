pub mod bit;
pub mod check;
pub mod operation;

#[derive(Copy, Clone)]
pub struct Board {
    player_board: u64,
    opponent_board: u64,
}

impl Board {
    pub fn new_default() -> Self {
        // 0000 0000
        // 0000 0000
        // 0000 0000
        // 000o p000
        // 000p o000
        // 0000 0000
        // 0000 0000
        // 0000 0000
        Board {
            player_board: 0x0000000810000000,
            opponent_board: 0x0000001008000000,
        }
    }
    pub fn new(player_board: u64, opponent_board: u64) -> Self {
        Board {
            player_board,
            opponent_board,
        }
    }
}
