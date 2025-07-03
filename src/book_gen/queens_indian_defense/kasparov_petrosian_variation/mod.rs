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
# use reco::book::queens_indian_defense::KASPAROV_PETROSIAN_VARIATION;
assert_eq!(KASPAROV_PETROSIAN_VARIATION.original_name(), "Queen's Indian Defense: Kasparov-Petrosian Variation");
```"#
)]
pub static KASPAROV_PETROSIAN_VARIATION: Variation = Variation {
    name: "Kasparov-Petrosian Variation",
    parent: Some(&super::QUEENS_INDIAN_DEFENSE),
    variations: &[
        &ANDERSSON_VARIATION,
        &CLASSICAL_VARIATION,
        &HEDGEHOG_VARIATION,
        &KASPAROV_ATTACK,
        &MAIN_LINE,
        &MARCO_DEFENSE,
        &MODERN_VARIATION,
        &PETROSIAN_ATTACK,
        &POLOVODIN_GAMBIT,
        &RASHKOVSKY_ATTACK,
        &ROMANISHIN_ATTACK,
    ],
    lines: &[Line {
        parent: &KASPAROV_PETROSIAN_VARIATION,
        code: Code {
            volume: Volume::E,
            category: Category(RangedU8::new_static::<1>()),
        },
        moves: &[
            Normal {
                role: Pawn,
                from: Square::D2,
                capture: None,
                to: Square::D4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::G8,
                capture: None,
                to: Square::F6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C2,
                capture: None,
                to: Square::C4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E7,
                capture: None,
                to: Square::E6,
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
                role: Pawn,
                from: Square::B7,
                capture: None,
                to: Square::B6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::A2,
                capture: None,
                to: Square::A3,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C8,
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
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(66729360891179520),
                    knight: Bitboard(144150372450304000),
                    bishop: Bitboard(2306405959167115300),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13542097580107759616),
                    white: Bitboard(203813565),
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
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 2,
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
pub mod andersson_variation;
pub use andersson_variation::ANDERSSON_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod hedgehog_variation;
pub use hedgehog_variation::HEDGEHOG_VARIATION;
pub mod kasparov_attack;
pub use kasparov_attack::KASPAROV_ATTACK;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod marco_defense;
pub use marco_defense::MARCO_DEFENSE;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod petrosian_attack;
pub use petrosian_attack::PETROSIAN_ATTACK;
pub mod polovodin_gambit;
pub use polovodin_gambit::POLOVODIN_GAMBIT;
pub mod rashkovsky_attack;
pub use rashkovsky_attack::RASHKOVSKY_ATTACK;
pub mod romanishin_attack;
pub use romanishin_attack::ROMANISHIN_ATTACK;
