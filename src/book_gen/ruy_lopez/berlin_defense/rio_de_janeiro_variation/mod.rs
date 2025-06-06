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
# use reco::book::ruy_lopez::berlin_defense::RIO_DE_JANEIRO_VARIATION;
assert_eq!(RIO_DE_JANEIRO_VARIATION.original_name(), "Ruy Lopez: Berlin Defense, Rio de Janeiro Variation");
```"#
)]
pub static RIO_DE_JANEIRO_VARIATION: Variation = Variation {
    name: "Rio de Janeiro Variation",
    parent: Some(&super::BERLIN_DEFENSE),
    variations: &[],
    lines: &[
        Line {
            parent: &RIO_DE_JANEIRO_VARIATION,
            code: Code {
                volume: Volume::C,
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
                    from: E7,
                    capture: None,
                    to: E5,
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
                    role: Bishop,
                    from: F1,
                    capture: None,
                    to: B5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
                Normal {
                    role: Knight,
                    from: F6,
                    capture: Some(Pawn),
                    to: E4,
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
                        pawn: Bitboard(67272588287600384),
                        knight: Bitboard(4398317043714),
                        bishop: Bitboard(292733984369016836),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(11384822850050326528),
                        white: Bitboard(8726308719),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
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
            parent: &RIO_DE_JANEIRO_VARIATION,
            code: Code {
                volume: Volume::C,
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
                    from: E7,
                    capture: None,
                    to: E5,
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
                    role: Bishop,
                    from: F1,
                    capture: None,
                    to: B5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
                Normal {
                    role: Knight,
                    from: F6,
                    capture: Some(Pawn),
                    to: E4,
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
                    role: Bishop,
                    from: F8,
                    capture: None,
                    to: E7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: E2,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: E4,
                    capture: None,
                    to: D6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: B5,
                    capture: Some(Knight),
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B7,
                    capture: Some(Bishop),
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D4,
                    capture: Some(Pawn),
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: D6,
                    capture: None,
                    to: B7,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: C3,
                    promotion: None,
                },
                Castle { king: E8, rook: H8 },
                Normal {
                    role: Rook,
                    from: F1,
                    capture: None,
                    to: E1,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B7,
                    capture: None,
                    to: C5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F3,
                    capture: None,
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: C5,
                    capture: None,
                    to: E6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: E6,
                    capture: Some(Knight),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: E3,
                    capture: Some(Knight),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C6,
                    capture: None,
                    to: C5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(66709655379830528),
                        knight: Bitboard(262144),
                        bishop: Bitboard(292733975913299968),
                        rook: Bitboard(2377900603251621905),
                        queen: Bitboard(576460752303427584),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7925490936421810176),
                        white: Bitboard(68854019921),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(0),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(14) {
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
