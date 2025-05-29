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
# use reco::book::kings_indian_defense::AVERBAKH_VARIATION;
assert_eq!(AVERBAKH_VARIATION.original_name(), "King's Indian Defense: Averbakh Variation");
```"#
)]
pub static AVERBAKH_VARIATION: Variation = Variation {
    name: "Averbakh Variation",
    parent: Some(&super::KINGS_INDIAN_DEFENSE),
    variations: &[
        &FLEXIBLE_DEFENSE,
        &SPANISH_DEFENSE,
        &GELLER_DEFENSE,
        &NC6_DEFENSE,
        &BENONI_DEFENSE,
        &MAIN_LINE,
        &MODERN_DEFENSE,
    ],
    lines: &[Line {
        parent: &AVERBAKH_VARIATION,
        code: Code {
            volume: Volume::E,
            category: Category::new_static::<73>(),
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
                from: G7,
                capture: None,
                to: G6,
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
                to: G7,
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
            Castle { king: E8, rook: H8 },
            Normal {
                role: Bishop,
                from: C1,
                capture: None,
                to: G5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(51589086045070080),
                    knight: Bitboard(144150372448206912),
                    bishop: Bitboard(306245049539104768),
                    rook: Bitboard(2377900603251622017),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387920),
                },
                ByColor {
                    black: Bitboard(8068031606666821632),
                    white: Bitboard(275347993561),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(129),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 3,
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
    }],
};
pub mod flexible_defense;
pub use flexible_defense::FLEXIBLE_DEFENSE;
pub mod spanish_defense;
pub use spanish_defense::SPANISH_DEFENSE;
pub mod geller_defense;
pub use geller_defense::GELLER_DEFENSE;
pub mod nc6_defense;
pub use nc6_defense::NC6_DEFENSE;
pub mod benoni_defense;
pub use benoni_defense::BENONI_DEFENSE;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod modern_defense;
pub use modern_defense::MODERN_DEFENSE;
