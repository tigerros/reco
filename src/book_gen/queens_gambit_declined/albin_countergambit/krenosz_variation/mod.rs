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
# use reco::book::queens_gambit_declined::albin_countergambit::KRENOSZ_VARIATION;
assert_eq!(KRENOSZ_VARIATION.original_name(), "Queen's Gambit Declined: Albin Countergambit, Krenosz Variation");
```"#
)]
pub static KRENOSZ_VARIATION: Variation = Variation {
    name: "Krenosz Variation",
    parent: Some(&super::ALBIN_COUNTERGAMBIT),
    variations: &[],
    lines: &[Line {
        parent: &KRENOSZ_VARIATION,
        code: Code {
            volume: Volume::D,
            category: Category(RangedU8::new_static::<0>()),
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
                role: Pawn,
                from: Square::D7,
                capture: None,
                to: Square::D5,
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
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::D4,
                capture: Some(Pawn),
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::D5,
                capture: None,
                to: Square::D4,
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
                role: Knight,
                from: Square::B8,
                capture: None,
                to: Square::C6,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::B1,
                capture: None,
                to: Square::D2,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C8,
                capture: None,
                to: Square::G4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::H2,
                capture: None,
                to: Square::H3,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::G4,
                capture: Some(Knight),
                to: Square::F3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::D2,
                capture: Some(Bishop),
                to: Square::F3,
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
                role: Bishop,
                from: Square::C1,
                capture: None,
                to: Square::D2,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D8,
                capture: None,
                to: Square::E7,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(65020788549382912),
                    knight: Bitboard(4611690416475996160),
                    bishop: Bitboard(33556512),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(4503599627370504),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(15129565871388753920),
                    white: Bitboard(68797103033),
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
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
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
