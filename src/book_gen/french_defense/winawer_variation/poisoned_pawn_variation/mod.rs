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
# use reco::book::french_defense::winawer_variation::POISONED_PAWN_VARIATION;
assert_eq!(POISONED_PAWN_VARIATION.original_name(), "French Defense: Winawer Variation, Poisoned Pawn Variation");
```"#
)]
pub static POISONED_PAWN_VARIATION: Variation = Variation {
    name: "Poisoned Pawn Variation",
    parent: Some(&super::WINAWER_VARIATION),
    variations: &[&MAIN_LINE, &PAOLI_VARIATION],
    lines: &[Line {
        parent: &POISONED_PAWN_VARIATION,
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
                role: Bishop,
                from: Square::F8,
                capture: None,
                to: Square::B4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E4,
                capture: None,
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C7,
                capture: None,
                to: Square::C5,
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
                from: Square::B4,
                capture: Some(Knight),
                to: Square::C3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::B2,
                capture: Some(Bishop),
                to: Square::C3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::G8,
                capture: None,
                to: Square::E7,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D1,
                capture: None,
                to: Square::G4,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D8,
                capture: None,
                to: Square::C7,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(63912532293051392),
                    knight: Bitboard(148618787703226432),
                    bishop: Bitboard(288230376151711780),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(1125900980584448),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(10950238662700302336),
                    white: Bitboard(69927822581),
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
            halfmoves: 3,
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
    }],
};
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod paoli_variation;
pub use paoli_variation::PAOLI_VARIATION;
