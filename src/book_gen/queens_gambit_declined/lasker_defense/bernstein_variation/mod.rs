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
# use reco::book::queens_gambit_declined::lasker_defense::BERNSTEIN_VARIATION;
assert_eq!(BERNSTEIN_VARIATION.original_name(), "Queen's Gambit Declined: Lasker Defense, Bernstein Variation");
```"#
)]
pub static BERNSTEIN_VARIATION: Variation = Variation {
    name: "Bernstein Variation",
    parent: Some(&super::LASKER_DEFENSE),
    variations: &[&MAR_DEL_PLATA_GAMBIT],
    lines: &[Line {
        parent: &BERNSTEIN_VARIATION,
        code: Code {
            volume: Volume::D,
            category: Category::new_static::<57>(),
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
                from: B1,
                capture: None,
                to: C3,
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
                role: Knight,
                from: G1,
                capture: None,
                to: F3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F6,
                capture: None,
                to: E4,
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
                role: Pawn,
                from: C4,
                capture: Some(Pawn),
                to: D5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: E4,
                capture: Some(Knight),
                to: C3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: B2,
                capture: Some(Knight),
                to: C3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E6,
                capture: Some(Pawn),
                to: D5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D1,
                capture: None,
                to: B3,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: E7,
                capture: None,
                to: D6,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(29132694584877312),
                    knight: Bitboard(144115188077953024),
                    bishop: Bitboard(288230376151711776),
                    rook: Bitboard(2377900603251622017),
                    queen: Bitboard(8796093153280),
                    king: Bitboard(4611686018427387920),
                },
                ByColor {
                    black: Bitboard(7451073676448890880),
                    white: Bitboard(137814449),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(129),
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
    }],
};
pub mod mar_del_plata_gambit;
pub use mar_del_plata_gambit::MAR_DEL_PLATA_GAMBIT;
