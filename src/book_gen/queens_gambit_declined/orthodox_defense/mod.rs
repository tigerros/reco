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
# use reco::book::queens_gambit_declined::ORTHODOX_DEFENSE;
assert_eq!(ORTHODOX_DEFENSE.original_name(), "Queen's Gambit Declined: Orthodox Defense");
```"#
)]
pub static ORTHODOX_DEFENSE: Variation = Variation {
    name: "Orthodox Defense",
    parent: Some(&super::QUEENS_GAMBIT_DECLINED),
    variations: &[
        &ALEKHINE_VARIATION,
        &BD3_LINE,
        &BOTVINNIK_VARIATION,
        &CAPABLANCA_SYSTEM,
        &CAPABLANCA_VARIATION,
        &CLASSICAL_VARIATION,
        &FIANCHETTO_VARIATION,
        &HENNEBERGER_VARIATION,
        &JANOWSKI_VARIATION,
        &MAIN_LINE,
        &PILLSBURY_VARIATION,
        &RAUZER_VARIATION,
        &RUBINSTEIN_ATTACK,
        &RUBINSTEIN_VARIATION,
        &SWISS,
    ],
    lines: &[Line {
        parent: &ORTHODOX_DEFENSE,
        code: Code {
            volume: Volume::D,
            category: Category::new_static::<6>(),
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
                role: Bishop,
                from: Square::F8,
                capture: None,
                to: Square::E7,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C1,
                capture: None,
                to: Square::G5,
                promotion: None,
            },
            Castle {
                king: Square::E8,
                rook: Square::H8,
            },
            Normal {
                role: Pawn,
                from: Square::E2,
                capture: None,
                to: Square::E3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::B8,
                capture: None,
                to: Square::D7,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(65038346368377600),
                    knight: Bitboard(2286984188133376),
                    bishop: Bitboard(292734250656989216),
                    rook: Bitboard(2377900603251622017),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387920),
                },
                ByColor {
                    black: Bitboard(7926106680113233920),
                    white: Bitboard(275082699705),
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
            castling_rights: Bitboard(129),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
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
    }],
};
pub mod alekhine_variation;
pub use alekhine_variation::ALEKHINE_VARIATION;
pub mod bd3_line;
pub use bd3_line::BD3_LINE;
pub mod botvinnik_variation;
pub use botvinnik_variation::BOTVINNIK_VARIATION;
pub mod capablanca_system;
pub use capablanca_system::CAPABLANCA_SYSTEM;
pub mod capablanca_variation;
pub use capablanca_variation::CAPABLANCA_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
pub mod henneberger_variation;
pub use henneberger_variation::HENNEBERGER_VARIATION;
pub mod janowski_variation;
pub use janowski_variation::JANOWSKI_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod pillsbury_variation;
pub use pillsbury_variation::PILLSBURY_VARIATION;
pub mod rauzer_variation;
pub use rauzer_variation::RAUZER_VARIATION;
pub mod rubinstein_attack;
pub use rubinstein_attack::RUBINSTEIN_ATTACK;
pub mod rubinstein_variation;
pub use rubinstein_variation::RUBINSTEIN_VARIATION;
pub mod swiss;
pub use swiss::SWISS;
