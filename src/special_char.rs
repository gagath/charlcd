//! Module containing special characters constant codes (Greek letters, math
//! symbols, etc.)
//!
//! Note: these characters only work on the HD44780 screen.

/// α
pub const ALPHA: u8 = 0xe0;
/// β
pub const BETA: u8 = 0xe2;
/// ε
pub const EPSILON: u8 = 0xe3;
/// μ
pub const MU: u8 = 0xe4;
/// σ
pub const SIGMA: u8 = 0xe5;
/// ρ
pub const RO: u8 = 0xe6;
/// θ
pub const THETA: u8 = 0xf2;
/// Ω
pub const OMEGA: u8 = 0xf4;
/// Σ
pub const SIGMA_UPPER: u8 = 0xf6;
/// π
pub const PI: u8 = 0xf7;

/// ✓
pub const SQRT: u8 = 0xe8;
/// ⁻¹
pub const INV: u8 = 0xe9;
/// ∞
pub const INFINITE: u8 = 0xf3;
/// ÷
pub const DIV: u8 = 0xfd;
/// x̅
pub const MEAN: u8 = 0xf8;

/// ·
pub const MEDIAN_DOT: u8 = 0xa5;
/// ▮
pub const BLOCK: u8 = 0xff;
