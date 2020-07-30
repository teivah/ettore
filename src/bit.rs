pub fn bytes_from_low_bits(n: i32) -> (i8, i8, i8, i8) {
    let mut i1: i8 = 0;
    let mut i2: i8 = 0;
    let mut i3: i8 = 0;
    let mut i4: i8 = 0;

    let mut index: u8 = 0;
    for i in 0..8 {
        if get_i32_bit(n, i) {
            i1 = set_i8_bit(i1, index);
        }
        index += 1;
    }

    index = 0;
    for i in 8..16 {
        if get_i32_bit(n, i) {
            i2 = set_i8_bit(i2, index);
        }
        index += 1;
    }

    index = 0;
    for i in 16..24 {
        if get_i32_bit(n, i) {
            i3 = set_i8_bit(i3, index);
        }
        index += 1;
    }

    index = 0;
    for i in 24..32 {
        if get_i32_bit(n, i) {
            i4 = set_i8_bit(i4, index);
        }
        index += 1;
    }

    return (i1, i2, i3, i4);
}

pub fn i32_from_bytes(i1: i8, i2: i8, i3: i8, i4: i8) -> i32 {
    let mut index: u8 = 0;
    let mut result: i32 = 0;

    for i in 0..8 {
        if get_i8_bit(i1, i) {
            result = set_i32_bit(result, index);
        }
        index += 1;
    }

    for i in 0..8 {
        if get_i8_bit(i2, i) {
            result = set_i32_bit(result, index);
        }
        index += 1;
    }

    for i in 0..8 {
        if get_i8_bit(i3, i) {
            result = set_i32_bit(result, index);
        }
        index += 1;
    }

    for i in 0..8 {
        if get_i8_bit(i4, i) {
            result = set_i32_bit(result, index);
        }
        index += 1;
    }

    return result;
}

fn get_i8_bit(input: i8, n: u8) -> bool {
    input & (1 << n) != 0
}

fn get_i32_bit(input: i32, n: u8) -> bool {
    input & (1 << n) != 0
}

fn set_i8_bit(n: i8, i: u8) -> i8 {
    n | (1 << i)
}

fn set_i32_bit(n: i32, i: u8) -> i32 {
    n | (1 << i)
}
