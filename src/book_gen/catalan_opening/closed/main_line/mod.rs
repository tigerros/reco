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
# use reco::book::catalan_opening::closed::MAIN_LINE;
assert_eq!(MAIN_LINE.original_name(), "Catalan Opening: Closed, Main Line");
```"#
)]
pub static MAIN_LINE: Variation = Variation {
    name: "Main Line",
    parent: Some(&super::CLOSED),
    variations: &[],
    lines: &[
        Line {
            parent: &MAIN_LINE,
            code: Code {
                volume: Volume::E,
                category: Category::new_static::<9>(),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
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
                    role: Pawn,
                    from: C2,
                    capture: None,
                    to: C4,
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
                    role: Knight,
                    from: G1,
                    capture: None,
                    to: F3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: G2,
                    capture: None,
                    to: G3,
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
                    role: Bishop,
                    from: F1,
                    capture: None,
                    to: G2,
                    promotion: None,
                },
                Castle { king: E8, rook: H8 },
                Castle { king: E1, rook: H1 },
                Normal {
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: C2,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C7,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: D2,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63916844511179520),
                        knight: Bitboard(2286984187873280),
                        bishop: Bitboard(292733975779098628),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7924985178252902400),
                        white: Bitboard(207683429),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(0),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
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
            parent: &MAIN_LINE,
            code: Code {
                volume: Volume::E,
                category: Category::new_static::<9>(),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
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
                    role: Pawn,
                    from: C2,
                    capture: None,
                    to: C4,
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
                    role: Knight,
                    from: G1,
                    capture: None,
                    to: F3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: G2,
                    capture: None,
                    to: G3,
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
                    role: Bishop,
                    from: F1,
                    capture: None,
                    to: G2,
                    promotion: None,
                },
                Castle { king: E8, rook: H8 },
                Castle { king: E1, rook: H1 },
                Normal {
                    role: Pawn,
                    from: C7,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: C2,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B7,
                    capture: None,
                    to: B6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C8,
                    capture: None,
                    to: B7,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63356093849445120),
                        knight: Bitboard(2286984187873280),
                        bishop: Bitboard(5066549580808196),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7636757001124446208),
                        white: Bitboard(476114789),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(0),
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
        Line {
            parent: &MAIN_LINE,
            code: Code {
                volume: Volume::E,
                category: Category::new_static::<9>(),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
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
                    role: Pawn,
                    from: C2,
                    capture: None,
                    to: C4,
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
                    role: Knight,
                    from: G1,
                    capture: None,
                    to: F3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: G2,
                    capture: None,
                    to: G3,
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
                    role: Bishop,
                    from: F1,
                    capture: None,
                    to: G2,
                    promotion: None,
                },
                Castle { king: E8, rook: H8 },
                Castle { king: E1, rook: H1 },
                Normal {
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: C2,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C7,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: F4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B7,
                    capture: None,
                    to: B6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C8,
                    capture: None,
                    to: B7,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D5,
                    capture: Some(Pawn),
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: D2,
                    capture: Some(Pawn),
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F6,
                    capture: Some(Knight),
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: C2,
                    capture: Some(Knight),
                    to: E4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63356059221271296),
                        knight: Bitboard(2251799815782400),
                        bishop: Bitboard(5066550117679104),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752571858944),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7636721782392619008),
                        white: Bitboard(1012982625),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(0),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(12) {
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
