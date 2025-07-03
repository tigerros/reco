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
use core::unreachable;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use deranged::RangedU8;
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
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square;
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
                category: Category(RangedU8::new_static::<6>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Knight,
                    from: Square::F6,
                    capture: Some(Pawn),
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
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
                ) {
                    board
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
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
                category: Category(RangedU8::new_static::<6>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Knight,
                    from: Square::F6,
                    capture: Some(Pawn),
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: Square::D1,
                    capture: None,
                    to: Square::E2,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::E4,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B5,
                    capture: Some(Knight),
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B7,
                    capture: Some(Bishop),
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D4,
                    capture: Some(Pawn),
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::D6,
                    capture: None,
                    to: Square::B7,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B1,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
                Castle {
                    king: Square::E8,
                    rook: Square::H8,
                },
                Normal {
                    role: Rook,
                    from: Square::F1,
                    capture: None,
                    to: Square::E1,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B7,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::F3,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::C5,
                    capture: None,
                    to: Square::E6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::E3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::E6,
                    capture: Some(Knight),
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::E3,
                    capture: Some(Knight),
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C6,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
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
                ) {
                    board
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
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
