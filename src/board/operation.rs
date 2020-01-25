use crate::board::bit::{position_to_bit, transfer};
use crate::board::Board;

impl Board {
    pub fn make_legal_board(&self) -> u64 {
        // 7e == 0111 1110
        let horizontal_watch_board: u64 = self.opponent_board & 0x7e7e7e7e7e7e7e7e;

        let vertical_watch_board: u64 = self.opponent_board & 0x00FFFFFFFFFFFF00;

        let all_side_watch_board: u64 = self.opponent_board & 0x007e7e7e7e7e7e00;

        let blank_board: u64 = !(self.player_board | self.opponent_board);
        let mut legal_board = 0x0000000000000000;

        //左
        let mut tmp = horizontal_watch_board & (self.player_board << 1);
        tmp |= horizontal_watch_board & (tmp << 1);
        tmp |= horizontal_watch_board & (tmp << 1);
        tmp |= horizontal_watch_board & (tmp << 1);
        tmp |= horizontal_watch_board & (tmp << 1);
        tmp |= horizontal_watch_board & (tmp << 1);
        legal_board = blank_board & (tmp << 1);

        //右
        let mut tmp = horizontal_watch_board & (self.player_board >> 1);
        tmp |= horizontal_watch_board & (tmp >> 1);
        tmp |= horizontal_watch_board & (tmp >> 1);
        tmp |= horizontal_watch_board & (tmp >> 1);
        tmp |= horizontal_watch_board & (tmp >> 1);
        tmp |= horizontal_watch_board & (tmp >> 1);
        legal_board |= blank_board & (tmp >> 1);

        //上
        let mut tmp = vertical_watch_board & (self.player_board << 8);
        tmp |= vertical_watch_board & (tmp << 8);
        tmp |= vertical_watch_board & (tmp << 8);
        tmp |= vertical_watch_board & (tmp << 8);
        tmp |= vertical_watch_board & (tmp << 8);
        tmp |= vertical_watch_board & (tmp << 8);
        legal_board |= blank_board & (tmp << 8);

        //下
        let mut tmp = vertical_watch_board & (self.player_board >> 8);
        tmp |= vertical_watch_board & (tmp >> 8);
        tmp |= vertical_watch_board & (tmp >> 8);
        tmp |= vertical_watch_board & (tmp >> 8);
        tmp |= vertical_watch_board & (tmp >> 8);
        tmp |= vertical_watch_board & (tmp >> 8);
        legal_board |= blank_board & (tmp >> 8);

        //右斜め上
        let mut tmp = all_side_watch_board & (self.player_board << 7);
        tmp |= all_side_watch_board & (tmp << 7);
        tmp |= all_side_watch_board & (tmp << 7);
        tmp |= all_side_watch_board & (tmp << 7);
        tmp |= all_side_watch_board & (tmp << 7);
        tmp |= all_side_watch_board & (tmp << 7);
        legal_board |= blank_board & (tmp << 7);

        //左斜め上
        let mut tmp = all_side_watch_board & (self.player_board << 9);
        tmp |= all_side_watch_board & (tmp << 9);
        tmp |= all_side_watch_board & (tmp << 9);
        tmp |= all_side_watch_board & (tmp << 9);
        tmp |= all_side_watch_board & (tmp << 9);
        tmp |= all_side_watch_board & (tmp << 9);
        legal_board |= blank_board & (tmp << 9);

        //右斜め下
        let mut tmp = all_side_watch_board & (self.player_board >> 9);
        tmp |= all_side_watch_board & (tmp >> 9);
        tmp |= all_side_watch_board & (tmp >> 9);
        tmp |= all_side_watch_board & (tmp >> 9);
        tmp |= all_side_watch_board & (tmp >> 9);
        tmp |= all_side_watch_board & (tmp >> 9);
        legal_board |= blank_board & (tmp >> 9);

        //左斜め下
        let mut tmp = all_side_watch_board & (self.player_board >> 7);
        tmp |= all_side_watch_board & (tmp >> 7);
        tmp |= all_side_watch_board & (tmp >> 7);
        tmp |= all_side_watch_board & (tmp >> 7);
        tmp |= all_side_watch_board & (tmp >> 7);
        tmp |= all_side_watch_board & (tmp >> 7);
        legal_board |= blank_board & (tmp >> 7);

        legal_board
    }

    pub fn can_put(&self, put: u64) -> bool {
        let legal_board: u64 = self.make_legal_board();
        (put & legal_board) == put
    }

    pub fn reverse(&mut self, put: u64) {
        let mut rev: u64 = 0;
        for k in 0..8u64 {
            let mut rev_: u64 = 0;
            let mut mask: u64 = transfer(put, k);
            while (mask != 0) && ((mask & self.opponent_board) != 0) {
                rev_ |= mask;
                mask = transfer(mask, k)
            }
            if (mask & self.player_board) != 0 {
                rev |= rev_
            }
        }

        self.player_board ^= put | rev;
        self.opponent_board ^= rev;
    }
}

#[test]
fn make_legal_board_test() {
    let lb = Board::new_default().make_legal_board();
    // 0000 0000 .. 00
    // 0000 0000 .. 00
    // 0001 0000 .. 10
    // 0010 0000 .. 20
    // 0000 0100 .. 04
    // 0000 1000 .. 08
    // 0000 0000 .. 00
    // 0000 0000 .. 00
    assert_eq!(lb, 0x00_00_10_20_04_08_00_00);
}

#[test]
fn can_put_test() {
    let b = Board::new_default();
    let can_put = b.can_put(position_to_bit("D", 3));
    assert_eq!(can_put, true);

    let can_put = b.can_put(position_to_bit("D", 4));
    assert_eq!(can_put, false);
}

#[test]
fn reverse_test() {
    let mut b = Board::new_default();
    let put = position_to_bit("D", 3);
    let can_put = b.can_put(put);

    assert_eq!(can_put, true);
    b.reverse(put);

    // 0000 0000
    // 0000 0000
    // 0001 0000
    // 0001 1000
    // 0001 0000
    // 0000 0000
    // 0000 0000
    // 0000 0000
    assert_eq!(b.player_board, 0x0000101810000000);
}
