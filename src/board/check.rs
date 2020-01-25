use crate::board::bit::bit_count;
use crate::board::Board;

impl Board {
    pub fn is_pass(&self) -> bool {
        let player_legal_board = self.make_legal_board();

        let opponent_legal_board = {
            let player_board = self.opponent_board;
            let opponent_board = self.player_board;
            let tmp_board = Board::new(player_board, opponent_board);
            tmp_board.make_legal_board()
        };

        let unavoidable_pass_bit = 0x0000000000000000;
        player_legal_board == unavoidable_pass_bit && opponent_legal_board != unavoidable_pass_bit
    }

    pub fn is_game_finished(&self) -> bool {
        let player_legal_board = self.make_legal_board();

        let mut tmp_board = Board::new_default();
        tmp_board.player_board = self.player_board;
        tmp_board.opponent_board = self.opponent_board;

        let opponent_legal_board = tmp_board.make_legal_board();
        let unavoidable_pass_bit = 0x0000000000000000;
        player_legal_board == unavoidable_pass_bit && opponent_legal_board == unavoidable_pass_bit
    }

    pub fn result(&self) -> (u64, u64, &str) {
        let player_score = bit_count(self.player_board);
        let opponent_score = bit_count(self.opponent_board);
        let winner = if opponent_score < player_score {
            "player"
        } else {
            "opponent"
        };

        return (player_score, opponent_score, winner);
    }
}

#[test]
fn is_pass_test() {
    // w -> player, b -> opponent
    // 0000 0000
    // 0000 0000
    // bwww wwww
    // bbww wwww
    // bwbb wwww
    // 0wwb wwww
    // 00ww ww00
    // 00ww ww00
    let player_board = 0x00_00_7f_3f_4f_6f_3d_3d;
    let opponent_board = 0x00_00_80_c0_b0_10_00_00;

    let mut b = Board::new(player_board, opponent_board);
    assert_eq!(b.is_pass(), true);
}

#[test]
fn is_game_finished_test() {
    // w -> player, b -> opponent
    // 0000 0000
    // 0000 0000
    // 0000 0000
    // 000w w000
    // 000w w000
    // 0000 0000
    // 0000 0000
    // 0000 0000
    let p = 0x00_00_00_18_18_00_00_00;
    let o = 0;
    let mut b = Board::new(p, o);

    assert_eq!(Board::new_default().is_game_finished(), false);
}

#[test]
fn result_test() {
    let p = 0x00_00_00_00_00_00_11_11;
    let o = 0x00_00_00_00_00_00_00_00;
    assert_eq!(Board::new(p, o).result(), (4, 0, "player"));

    let o = 0x00_00_00_00_11_11_00_00;
    assert_eq!(Board::new(p, o).result(), (4, 4, "opponent"));
}
