/// X axis mirror of a custom LCD character
pub const fn mirror_x(src: [u8; 8]) -> [u8; 8] {
    // const fn do not support for loops currently, hence manual unroll
    [
        src[0].reverse_bits() >> 3,
        src[1].reverse_bits() >> 3,
        src[2].reverse_bits() >> 3,
        src[3].reverse_bits() >> 3,
        src[4].reverse_bits() >> 3,
        src[5].reverse_bits() >> 3,
        src[6].reverse_bits() >> 3,
        src[7].reverse_bits() >> 3,
    ]
}

/// Y axis mirror of a custom LCD character
pub const fn mirror_y(src: [u8; 8]) -> [u8; 8] {
    // const fn do not support for loops currently, hence manual unroll
    [
        src[7], src[6], src[5], src[4], src[3], src[2], src[1], src[0],
    ]
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const RIGHT_ARROW: [u8; 8] = [
    0b00000,
    0b01000,
    0b01100,
    0b01110,
    0b01100,
    0b01000,
    0b00000,
    0b00000,
];

pub const LEFT_ARROW: [u8; 8] = mirror_x(RIGHT_ARROW);

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const UP_ARROW: [u8; 8] = [
    0b00000,
    0b00000,
    0b00100,
    0b01110,
    0b11111,
    0b00000,
    0b00000,
    0b00000,
];

pub const DOWN_ARROW: [u8; 8] = mirror_y(UP_ARROW);
