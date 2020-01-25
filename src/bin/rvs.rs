use rvs::board::bit::position_to_bit;
use rvs::board::Board;

fn main() {
    let mut b = Board::new_default();
    let put = position_to_bit("D", 3);
    let can_put = b.can_put(put);

    assert_eq!(can_put, true);
    b.reverse(put)
}
