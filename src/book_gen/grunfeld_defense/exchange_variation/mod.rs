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
# use reco::book::grunfeld_defense::EXCHANGE_VARIATION;
assert_eq!(EXCHANGE_VARIATION.original_name(), "Grünfeld Defense: Exchange Variation");
```"#
)]
pub static EXCHANGE_VARIATION: Variation = Variation {
    name: "Exchange Variation",
    parent: Some(&super::GRUNFELD_DEFENSE),
    variations: &[
        &CLASSICAL_VARIATION,
        &LARSEN_VARIATION,
        &MODERN_EXCHANGE_VARIATION,
        &NADANIAN_ATTACK,
        &SEVILLE_VARIATION,
        &SIMAGINS_IMPROVED_VARIATION,
        &SIMAGINS_LESSER_VARIATION,
        &SOKOLSKY_VARIATION,
        &SPASSKY_VARIATION,
    ],
    lines: &[Line {
        parent: &EXCHANGE_VARIATION,
        code: Code {
            volume: Volume::D,
            category: Category::new_static::<85>(),
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
                role: Pawn,
                from: D7,
                capture: None,
                to: D5,
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
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(51580289616507648),
                    knight: Bitboard(144115222435856448),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13814580785086201856),
                    white: Bitboard(134542333),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
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
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod larsen_variation;
pub use larsen_variation::LARSEN_VARIATION;
pub mod modern_exchange_variation;
pub use modern_exchange_variation::MODERN_EXCHANGE_VARIATION;
pub mod nadanian_attack;
pub use nadanian_attack::NADANIAN_ATTACK;
pub mod seville_variation;
pub use seville_variation::SEVILLE_VARIATION;
pub mod simagins_improved_variation;
pub use simagins_improved_variation::SIMAGINS_IMPROVED_VARIATION;
pub mod simagins_lesser_variation;
pub use simagins_lesser_variation::SIMAGINS_LESSER_VARIATION;
pub mod sokolsky_variation;
pub use sokolsky_variation::SOKOLSKY_VARIATION;
pub mod spassky_variation;
pub use spassky_variation::SPASSKY_VARIATION;
