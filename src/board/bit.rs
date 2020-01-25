pub fn position_to_bit(x: &str, y: u64) -> u64 {
    let a1: u64 = 0x8000000000000000;
    let mask = match x {
        "A" => a1,
        "B" => a1 >> 1,
        "C" => a1 >> 2,
        "D" => a1 >> 3,
        "E" => a1 >> 4,
        "F" => a1 >> 5,
        "G" => a1 >> 6,
        "H" => a1 >> 7,
        _ => unimplemented!(),
    };

    mask >> ((y - 1) * 8)
}

#[test]
fn position_to_bit_test() {
    // move x only
    assert_eq!(position_to_bit("A", 1), 0x8000000000000000, "A");
    assert_eq!(position_to_bit("B", 1), 0x4000000000000000, "B");
    assert_eq!(position_to_bit("C", 1), 0x2000000000000000, "C");
    assert_eq!(position_to_bit("D", 1), 0x1000000000000000, "D");
    assert_eq!(position_to_bit("E", 1), 0x0800000000000000, "E");
    assert_eq!(position_to_bit("F", 1), 0x0400000000000000, "F");
    assert_eq!(position_to_bit("G", 1), 0x0200000000000000, "G");
    assert_eq!(position_to_bit("H", 1), 0x0100000000000000, "H");

    // move y only
    assert_eq!(position_to_bit("A", 2), 0x0080000000000000, "2");
    assert_eq!(position_to_bit("A", 3), 0x0000800000000000, "3");
    assert_eq!(position_to_bit("A", 4), 0x0000008000000000, "4");
    assert_eq!(position_to_bit("A", 5), 0x0000000080000000, "5");
    assert_eq!(position_to_bit("A", 6), 0x0000000000800000, "6");
    assert_eq!(position_to_bit("A", 7), 0x0000000000008000, "7");
    assert_eq!(position_to_bit("A", 8), 0x0000000000000080, "8");
}

pub(crate) fn transfer(put: u64, k: u64) -> u64 {
    match k {
        0 => (put << 8) & 0xffffffffffffff00, //上
        1 => (put << 7) & 0x7f7f7f7f7f7f7f00, //右上
        2 => (put >> 1) & 0x7f7f7f7f7f7f7f7f, //右
        3 => (put >> 9) & 0x007f7f7f7f7f7f7f, //右下
        4 => (put >> 8) & 0x00ffffffffffffff, //下
        5 => (put >> 7) & 0x00fefefefefefefe, //左下
        6 => (put << 1) & 0xfefefefefefefefe, //左
        7 => (put << 9) & 0xfefefefefefefe00, //左上
        _ => 0,
    }
}

#[test]
fn transfer_test() {
    let put = position_to_bit("D", 3);
    assert_eq!(put, 0x00_00_10_00_00_00_00_00);

    assert_eq!(transfer(put, 0), 0x00_10_00_00_00_00_00_00);
    assert_eq!(transfer(put, 1), 0x00_08_00_00_00_00_00_00);
    assert_eq!(transfer(put, 2), 0x00_00_08_00_00_00_00_00);
    assert_eq!(transfer(put, 3), 0x00_00_00_08_00_00_00_00);
    assert_eq!(transfer(put, 4), 0x00_00_00_10_00_00_00_00);
    assert_eq!(transfer(put, 5), 0x00_00_00_20_00_00_00_00);
    assert_eq!(transfer(put, 6), 0x00_00_20_00_00_00_00_00);
    assert_eq!(transfer(put, 7), 0x00_20_00_00_00_00_00_00);
    assert_eq!(transfer(put, 8), 0x00_00_00_00_00_00_00_00);
}

pub(crate) fn bit_count(num: u64) -> u64 {
    let board_size = 64;
    let mut mask: u64 = 0x8000000000000000;
    let mut count = 0;

    for _ in 0..board_size {
        if mask & num != 0 {
            count += 1
        }
        mask = mask >> 1
    }
    count
}

#[test]
fn bit_count_test() {
    assert_eq!(bit_count(1), 1);
    assert_eq!(bit_count(3), 2);
    assert_eq!(bit_count(0x00_00_00_18_18_00_00_00), 4);
    assert_eq!(bit_count(0xff_ff_ff_ff_ff_ff_ff_ff), 64);
}
