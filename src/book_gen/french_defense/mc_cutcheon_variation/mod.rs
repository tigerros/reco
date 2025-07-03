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
# use reco::book::french_defense::MC_CUTCHEON_VARIATION;
assert_eq!(MC_CUTCHEON_VARIATION.original_name(), "French Defense: McCutcheon Variation");
```"#
)]
pub static MC_CUTCHEON_VARIATION: Variation = Variation {
    name: "McCutcheon Variation",
    parent: Some(&super::FRENCH_DEFENSE),
    variations: &[
        &ADVANCE_VARIATION,
        &BERNSTEIN_VARIATION,
        &BOGOLJUBOW_VARIATION,
        &CHIGORIN_VARIATION,
        &DR_OLLAND_VARIATION,
        &DURAS_VARIATION,
        &EXCHANGE_VARIATION,
        &GRIGORIEV_VARIATION,
        &JANOWSKI_VARIATION,
        &LASKER_VARIATION,
        &TARTAKOWER_VARIATION,
        &WOLF_GAMBIT,
    ],
    lines: &[Line {
        parent: &MC_CUTCHEON_VARIATION,
        code: Code {
            volume: Volume::C,
            category: Category(RangedU8::new_static::<1>()),
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
                to: Square::E6,
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
                role: Pawn,
                from: Square::D7,
                capture: None,
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::B1,
                capture: None,
                to: Square::C3,
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
                role: Bishop,
                from: Square::C1,
                capture: None,
                to: Square::G5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F8,
                capture: None,
                to: Square::B4,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(65038346568656640),
                    knight: Bitboard(144150372448206912),
                    bishop: Bitboard(288230651063173152),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(11522230982602129408),
                    white: Bitboard(275280881657),
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
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 4,
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
pub mod advance_variation;
pub use advance_variation::ADVANCE_VARIATION;
pub mod bernstein_variation;
pub use bernstein_variation::BERNSTEIN_VARIATION;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod chigorin_variation;
pub use chigorin_variation::CHIGORIN_VARIATION;
pub mod dr_olland_variation;
pub use dr_olland_variation::DR_OLLAND_VARIATION;
pub mod duras_variation;
pub use duras_variation::DURAS_VARIATION;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod grigoriev_variation;
pub use grigoriev_variation::GRIGORIEV_VARIATION;
pub mod janowski_variation;
pub use janowski_variation::JANOWSKI_VARIATION;
pub mod lasker_variation;
pub use lasker_variation::LASKER_VARIATION;
pub mod tartakower_variation;
pub use tartakower_variation::TARTAKOWER_VARIATION;
pub mod wolf_gambit;
pub use wolf_gambit::WOLF_GAMBIT;
