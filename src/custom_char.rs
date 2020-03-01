//! Create custom characters by defining their pixels.
//!
//! A custom character is considered as a 8x5 pixel array.
//!
//! You can declare a custom character by defining an array of type `[u8; 8]`
//! where:
//!
//! - each byte represents a line;
//! - each bit in the byte represent a pixel.
//!
//! Only the 5 lower bits of the byte are used, because the character width is
//! 5 pixels.
//!
//! # Example
//!
//! ```
//! /// ▸
//! #[cfg_attr(rustfmt, rustfmt_skip)]
//! pub const RIGHT_TRIANGLE: [u8; 8] = [
//!     0b00000,
//!     0b01000,
//!     0b01100,
//!     0b01110,
//!     0b01100,
//!     0b01000,
//!     0b00000,
//!     0b00000,
//! ];
//! ```
//!
//! The `#[cfg_attr(rustfmt, rustfmt_skip)]` part is required in order to
//! avoid `rustfmt` put each item after the next one so that we cannot see the
//! visual pixel representation of the custom character anymore.
//!
//! The custom character can then be put into the screen's memory by using the
//! [`Screen::custom_char`][super::Screen::custom_char] function.

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

/// ▸
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const RIGHT_TRIANGLE: [u8; 8] = [
    0b00000,
    0b01000,
    0b01100,
    0b01110,
    0b01100,
    0b01000,
    0b00000,
    0b00000,
];

/// ◂
pub const LEFT_TRIANGLE: [u8; 8] = mirror_x(RIGHT_TRIANGLE);

/// ▴
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const UP_TRIANGLE: [u8; 8] = [
    0b00000,
    0b00000,
    0b00100,
    0b01110,
    0b11111,
    0b00000,
    0b00000,
    0b00000,
];

/// ▾
pub const DOWN_TRIANGLE: [u8; 8] = mirror_y(UP_TRIANGLE);
