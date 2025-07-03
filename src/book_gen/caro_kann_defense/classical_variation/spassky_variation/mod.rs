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
# use reco::book::caro_kann_defense::classical_variation::SPASSKY_VARIATION;
assert_eq!(SPASSKY_VARIATION.original_name(), "Caro-Kann Defense: Classical Variation, Spassky Variation");
```"#
)]
pub static SPASSKY_VARIATION: Variation = Variation {
    name: "Spassky Variation",
    parent: Some(&super::CLASSICAL_VARIATION),
    variations: &[],
    lines: &[Line {
        parent: &SPASSKY_VARIATION,
        code: Code {
            volume: Volume::B,
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
                from: Square::C7,
                capture: None,
                to: Square::C6,
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
                to: Square::D2,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::D5,
                capture: Some(Pawn),
                to: Square::E4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::D2,
                capture: Some(Pawn),
                to: Square::E4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C8,
                capture: None,
                to: Square::F5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::E4,
                capture: None,
                to: Square::G3,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F5,
                capture: None,
                to: Square::G6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::H2,
                capture: None,
                to: Square::H4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::H7,
                capture: None,
                to: Square::H6,
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
                to: Square::D7,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::H4,
                capture: None,
                to: Square::H5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(32515307746649856),
                    knight: Bitboard(4613937818247364608),
                    bishop: Bitboard(2305913377957871652),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(17977177841858510848),
                    white: Bitboard(549896349629),
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
            halfmoves: 0,
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
