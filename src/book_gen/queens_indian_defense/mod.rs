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
# use reco::book::QUEENS_INDIAN_DEFENSE;
assert_eq!(QUEENS_INDIAN_DEFENSE.original_name(), "Queen's Indian Defense");
```"#
)]
pub static QUEENS_INDIAN_DEFENSE: Variation = Variation {
    name: "Queen's Indian Defense",
    parent: None,
    variations: &[
        &ANTI_QUEENS_INDIAN_SYSTEM,
        &AVERBAKH_VARIATION,
        &BUERGER_VARIATION,
        &CAPABLANCA_VARIATION,
        &CLASSICAL_VARIATION,
        &EUWE_VARIATION,
        &FIANCHETTO_VARIATION,
        &KASPAROV_VARIATION,
        &KASPAROV_PETROSIAN_VARIATION,
        &MILES_VARIATION,
        &OPOCENSKY_VARIATION,
        &PETROSIAN_VARIATION,
        &RIUMIN_VARIATION,
        &SPASSKY_SYSTEM,
        &TRADITIONAL_VARIATION,
        &YATES_VARIATION,
    ],
    lines: &[Line {
        parent: &QUEENS_INDIAN_DEFENSE,
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
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(66729360891114240),
                    knight: Bitboard(144150372450041858),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13829765006306050048),
                    white: Bitboard(203486143),
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
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
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
pub mod anti_queens_indian_system;
pub use anti_queens_indian_system::ANTI_QUEENS_INDIAN_SYSTEM;
pub mod averbakh_variation;
pub use averbakh_variation::AVERBAKH_VARIATION;
pub mod buerger_variation;
pub use buerger_variation::BUERGER_VARIATION;
pub mod capablanca_variation;
pub use capablanca_variation::CAPABLANCA_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod euwe_variation;
pub use euwe_variation::EUWE_VARIATION;
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
pub mod kasparov_variation;
pub use kasparov_variation::KASPAROV_VARIATION;
pub mod kasparov_petrosian_variation;
pub use kasparov_petrosian_variation::KASPAROV_PETROSIAN_VARIATION;
pub mod miles_variation;
pub use miles_variation::MILES_VARIATION;
pub mod opocensky_variation;
pub use opocensky_variation::OPOCENSKY_VARIATION;
pub mod petrosian_variation;
pub use petrosian_variation::PETROSIAN_VARIATION;
pub mod riumin_variation;
pub use riumin_variation::RIUMIN_VARIATION;
pub mod spassky_system;
pub use spassky_system::SPASSKY_SYSTEM;
pub mod traditional_variation;
pub use traditional_variation::TRADITIONAL_VARIATION;
pub mod yates_variation;
pub use yates_variation::YATES_VARIATION;
