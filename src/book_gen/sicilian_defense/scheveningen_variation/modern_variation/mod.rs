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
# use reco::book::sicilian_defense::scheveningen_variation::MODERN_VARIATION;
assert_eq!(MODERN_VARIATION.original_name(), "Sicilian Defense: Scheveningen Variation, Modern Variation");
```"#
)]
pub static MODERN_VARIATION: Variation = Variation {
    name: "Modern Variation",
    parent: Some(&super::SCHEVENINGEN_VARIATION),
    variations: &[],
    lines: &[
        Line {
            parent: &MODERN_VARIATION,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<83>(),
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
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
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
                    from: B8,
                    capture: None,
                    to: C6,
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
                    from: F1,
                    capture: None,
                    to: E2,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63921208260880128),
                        knight: Bitboard(39582553079808),
                        bishop: Bitboard(2594073385365409796),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13682846063579365376),
                        white: Bitboard(402978717),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 2,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
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
            parent: &MODERN_VARIATION,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<83>(),
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
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
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
                    from: F1,
                    capture: None,
                    to: E2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: F8,
                    capture: None,
                    to: E7,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
                Castle { king: E8, rook: H8 },
                Normal {
                    role: Pawn,
                    from: F2,
                    capture: None,
                    to: F4,
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
                    from: C1,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63921208797742848),
                        knight: Bitboard(39582553079808),
                        bishop: Bitboard(292733975780134912),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7922742140172500992),
                        white: Bitboard(940889961),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(0),
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
            parent: &MODERN_VARIATION,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<83>(),
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
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
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
                    from: F1,
                    capture: None,
                    to: E2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: F8,
                    capture: None,
                    to: E7,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
                Castle { king: E8, rook: H8 },
                Normal {
                    role: Pawn,
                    from: F2,
                    capture: None,
                    to: F4,
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
                    from: C1,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: D4,
                    capture: None,
                    to: B3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63921208797742848),
                        knight: Bitboard(39582418993152),
                        bishop: Bitboard(6755399442108416),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7636763563834474496),
                        white: Bitboard(806803305),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(0),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 4,
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
