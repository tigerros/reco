#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use crate::{Category, Code, Line, Variation, Volume};
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use core::num::NonZeroU32;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Color::{Black, White};
#[allow(
    unused_imports,
    clippy::enum_glob_use,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Move::*;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Role::{Bishop, King, Knight, Pawn, Queen, Rook};
#[allow(
    clippy::enum_glob_use,
    reason = "there's 64 variants in this enum, importing them all is stupid"
)]
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square::*;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::bitboard::Bitboard;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::board::Board;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::{ByColor, ByRole, Setup};
#[cfg_attr(
    feature = "alloc",
    doc = r#"```rust
# use reco::book::sicilian_defense::richter_rauzer_variation::NEO_MODERN_VARIATION;
assert_eq!(NEO_MODERN_VARIATION.original_name(), "Sicilian Defense: Richter-Rauzer Variation, Neo-Modern Variation");
```"#
)]
pub static NEO_MODERN_VARIATION: Variation = Variation {
    name: "Neo-Modern Variation",
    parent: Some(&super::RICHTER_RAUZER_VARIATION),
    variations: &[&EARLY_DEVIATIONS, &NYEZHMETDINOV_ATTACK],
    lines: &[
        Line {
            parent: &NEO_MODERN_VARIATION,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<67>(),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C7,
                    capture: None,
                    to: C5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: G1,
                    capture: None,
                    to: F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C5,
                    capture: Some(Pawn),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F3,
                    capture: Some(Pawn),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: G5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: D2,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: A7,
                    capture: None,
                    to: A6,
                    promotion: None,
                },
                Castle { king: E1, rook: A1 },
                Normal {
                    role: Bishop,
                    from: C8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63640832795797248),
                        knight: Bitboard(39582553079808),
                        bishop: Bitboard(2308095083905286176),
                        rook: Bitboard(9295429630892703880),
                        queen: Bitboard(576460752303425536),
                        king: Bitboard(1152921504606846980),
                    },
                    ByColor {
                        black: Bitboard(13396587111776256000),
                        white: Bitboard(275280883628),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 2,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
                    fullmoves
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
            },
        },
        Line {
            parent: &NEO_MODERN_VARIATION,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<68>(),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C7,
                    capture: None,
                    to: C5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: G1,
                    capture: None,
                    to: F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C5,
                    capture: Some(Pawn),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F3,
                    capture: Some(Pawn),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: G5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: D2,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: A7,
                    capture: None,
                    to: A6,
                    promotion: None,
                },
                Castle { king: E1, rook: A1 },
                Normal {
                    role: Bishop,
                    from: C8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: F2,
                    capture: None,
                    to: F4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: F8,
                    capture: None,
                    to: E7,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63640833332659968),
                        knight: Bitboard(39582553079808),
                        bishop: Bitboard(6755674318962720),
                        rook: Bitboard(9295429630892703880),
                        queen: Bitboard(576460752303425536),
                        king: Bitboard(1152921504606846980),
                    },
                    ByColor {
                        black: Bitboard(11095247702189932544),
                        white: Bitboard(275817746348),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
                    fullmoves
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
            },
        },
    ],
};
pub mod early_deviations;
pub use early_deviations::EARLY_DEVIATIONS;
pub mod nyezhmetdinov_attack;
pub use nyezhmetdinov_attack::NYEZHMETDINOV_ATTACK;
