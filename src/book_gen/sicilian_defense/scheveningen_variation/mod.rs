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
# use reco::book::sicilian_defense::SCHEVENINGEN_VARIATION;
assert_eq!(SCHEVENINGEN_VARIATION.original_name(), "Sicilian Defense: Scheveningen Variation");
```"#
)]
pub static SCHEVENINGEN_VARIATION: Variation = Variation {
    name: "Scheveningen Variation",
    parent: Some(&super::SICILIAN_DEFENSE),
    variations: &[
        &DELAYED_KERES_ATTACK,
        &KERES_ATTACK,
        &CLASSICAL_VARIATION,
        &TAL_VARIATION,
        &FIANCHETTO_VARIATION,
        &MATANOVIC_ATTACK,
        &VITOLINS_VARIATION,
        &ENGLISH_ATTACK,
        &MODERN_VARIATION,
        &CLASSICAL_MAIN_LINE,
    ],
    lines: &[Line {
        parent: &SCHEVENINGEN_VARIATION,
        code: Code {
            volume: Volume::B,
            category: Category::new_static::<80>(),
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
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(63921208260880128),
                    knight: Bitboard(144150372582424576),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13826956853608710144),
                    white: Bitboard(402974653),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
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
pub mod delayed_keres_attack;
pub use delayed_keres_attack::DELAYED_KERES_ATTACK;
pub mod keres_attack;
pub use keres_attack::KERES_ATTACK;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod tal_variation;
pub use tal_variation::TAL_VARIATION;
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
pub mod matanovic_attack;
pub use matanovic_attack::MATANOVIC_ATTACK;
pub mod vitolins_variation;
pub use vitolins_variation::VITOLINS_VARIATION;
pub mod english_attack;
pub use english_attack::ENGLISH_ATTACK;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod classical_main_line;
pub use classical_main_line::CLASSICAL_MAIN_LINE;
