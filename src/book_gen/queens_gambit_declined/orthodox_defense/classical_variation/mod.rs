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
# use reco::book::queens_gambit_declined::orthodox_defense::CLASSICAL_VARIATION;
assert_eq!(CLASSICAL_VARIATION.original_name(), "Queen's Gambit Declined: Orthodox Defense, Classical Variation");
```"#
)]
pub static CLASSICAL_VARIATION: Variation = Variation {
    name: "Classical Variation",
    parent: Some(&super::ORTHODOX_DEFENSE),
    variations: &[],
    lines: &[
        Line {
            parent: &CLASSICAL_VARIATION,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<68>(),
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
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D5,
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
                    from: B1,
                    capture: None,
                    to: C3,
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
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: G5,
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
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
                Castle { king: E8, rook: H8 },
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
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Rook,
                    from: A1,
                    capture: None,
                    to: C1,
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
                    from: F1,
                    capture: None,
                    to: D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D5,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: D3,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F6,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: G5,
                    capture: Some(Bishop),
                    to: E7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D8,
                    capture: Some(Bishop),
                    to: E7,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
                Normal {
                    role: Knight,
                    from: D5,
                    capture: Some(Knight),
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Rook,
                    from: C1,
                    capture: Some(Knight),
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E6,
                    capture: None,
                    to: E5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63899286614631168),
                        knight: Bitboard(2251799815782400),
                        bishop: Bitboard(288230376218820608),
                        rook: Bitboard(2377900603251884064),
                        queen: Bitboard(4503599627370504),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7348471683751084032),
                        white: Bitboard(204792680),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(0),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(13) {
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
            parent: &CLASSICAL_VARIATION,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<68>(),
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
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D5,
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
                    role: Bishop,
                    from: F8,
                    capture: None,
                    to: E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: G5,
                    promotion: None,
                },
                Castle { king: E8, rook: H8 },
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Rook,
                    from: A1,
                    capture: None,
                    to: C1,
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
                    from: F1,
                    capture: None,
                    to: D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D5,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: D3,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F6,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: G5,
                    capture: Some(Bishop),
                    to: E7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D8,
                    capture: Some(Bishop),
                    to: E7,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
                Normal {
                    role: Knight,
                    from: D5,
                    capture: Some(Knight),
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Rook,
                    from: C1,
                    capture: Some(Knight),
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E6,
                    capture: None,
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: B1,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63899286614631168),
                        knight: Bitboard(2251799815782400),
                        bishop: Bitboard(288230376218820608),
                        rook: Bitboard(2377900603251884064),
                        queen: Bitboard(4503599627370498),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7348471683751084032),
                        white: Bitboard(204792674),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(0),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(13) {
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
            parent: &CLASSICAL_VARIATION,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<68>(),
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
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: C3,
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
                    from: C1,
                    capture: None,
                    to: G5,
                    promotion: None,
                },
                Castle { king: E8, rook: H8 },
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Rook,
                    from: A1,
                    capture: None,
                    to: C1,
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
                    from: F1,
                    capture: None,
                    to: D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D5,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: D3,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F6,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: G5,
                    capture: Some(Bishop),
                    to: E7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D8,
                    capture: Some(Bishop),
                    to: E7,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
                Normal {
                    role: Knight,
                    from: D5,
                    capture: Some(Knight),
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Rook,
                    from: C1,
                    capture: Some(Knight),
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E6,
                    capture: None,
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: C2,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63899286614631168),
                        knight: Bitboard(2251799815782400),
                        bishop: Bitboard(288230376218820608),
                        rook: Bitboard(2377900603251884064),
                        queen: Bitboard(4503599627371520),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7348471683751084032),
                        white: Bitboard(204793696),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(0),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(13) {
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
            parent: &CLASSICAL_VARIATION,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<69>(),
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
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D5,
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
                    from: B1,
                    capture: None,
                    to: C3,
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
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: G5,
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
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
                Castle { king: E8, rook: H8 },
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
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Rook,
                    from: A1,
                    capture: None,
                    to: C1,
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
                    from: F1,
                    capture: None,
                    to: D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D5,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: D3,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F6,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: G5,
                    capture: Some(Bishop),
                    to: E7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D8,
                    capture: Some(Bishop),
                    to: E7,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
                Normal {
                    role: Knight,
                    from: D5,
                    capture: Some(Knight),
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Rook,
                    from: C1,
                    capture: Some(Knight),
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E6,
                    capture: None,
                    to: E5,
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
                    from: D7,
                    capture: Some(Pawn),
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F3,
                    capture: Some(Knight),
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: E7,
                    capture: Some(Knight),
                    to: E5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63899217760936704),
                        knight: Bitboard(0),
                        bishop: Bitboard(288230376218820608),
                        rook: Bitboard(2377900603251884064),
                        queen: Bitboard(68719476744),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7341716284310028288),
                        white: Bitboard(68477800),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(0),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(15) {
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
