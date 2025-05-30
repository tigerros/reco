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
# use reco::book::queens_gambit_declined::TARTAKOWER_DEFENSE;
assert_eq!(TARTAKOWER_DEFENSE.original_name(), "Queen's Gambit Declined: Tartakower Defense");
```"#
)]
pub static TARTAKOWER_DEFENSE: Variation = Variation {
    name: "Tartakower Defense",
    parent: Some(&super::QUEENS_GAMBIT_DECLINED),
    variations: &[&EXCHANGE_VARIATION, &MAKOGONOV_EXCHANGE_VARIATION],
    lines: &[
        Line {
            parent: &TARTAKOWER_DEFENSE,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<58>(),
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
                Normal {
                    role: Pawn,
                    from: H7,
                    capture: None,
                    to: H6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: G5,
                    capture: None,
                    to: H4,
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
                    role: Pawn,
                    from: B7,
                    capture: None,
                    to: B6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(28589535907603200),
                        knight: Bitboard(144150372450304000),
                        bishop: Bitboard(292733977926565920),
                        rook: Bitboard(2377900603251622017),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(4611686018427387920),
                    },
                    ByColor {
                        black: Bitboard(8031521257914630144),
                        white: Bitboard(2352276409),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(129),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
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
            parent: &TARTAKOWER_DEFENSE,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<59>(),
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
                    role: Bishop,
                    from: F8,
                    capture: None,
                    to: E7,
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
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: G5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: H7,
                    capture: None,
                    to: H6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: G5,
                    capture: None,
                    to: H4,
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
                    role: Pawn,
                    from: B7,
                    capture: None,
                    to: B6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C4,
                    capture: Some(Pawn),
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F6,
                    capture: Some(Pawn),
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: H4,
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
                Normal {
                    role: Knight,
                    from: C3,
                    capture: Some(Knight),
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E6,
                    capture: Some(Knight),
                    to: D5,
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
                    role: Bishop,
                    from: C8,
                    capture: None,
                    to: E6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(28571943654449920),
                        knight: Bitboard(144115188077953024),
                        bishop: Bitboard(17592186044448),
                        rook: Bitboard(2377900603251622020),
                        queen: Bitboard(4503599627370504),
                        king: Bitboard(4611686018427387920),
                    },
                    ByColor {
                        black: Bitboard(7166794945087406080),
                        white: Bitboard(137421756),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(128),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 2,
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
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod makogonov_exchange_variation;
pub use makogonov_exchange_variation::MAKOGONOV_EXCHANGE_VARIATION;
