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
# use reco::book::french_defense::winawer_variation::poisoned_pawn_variation::PAOLI_VARIATION;
assert_eq!(PAOLI_VARIATION.original_name(), "French Defense: Winawer Variation, Poisoned Pawn Variation, Paoli Variation");
```"#
)]
pub static PAOLI_VARIATION: Variation = Variation {
    name: "Paoli Variation",
    parent: Some(&super::POISONED_PAWN_VARIATION),
    variations: &[],
    lines: &[Line {
        parent: &PAOLI_VARIATION,
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
            Normal {
                role: Queen,
                from: Square::G4,
                capture: Some(Pawn),
                to: Square::G7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: Square::H8,
                capture: None,
                to: Square::G8,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::G7,
                capture: Some(Pawn),
                to: Square::H7,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C5,
                capture: Some(Pawn),
                to: Square::D4,
                promotion: None,
            },
            Normal {
                role: King,
                from: Square::E1,
                capture: None,
                to: Square::D1,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(9869319584736256),
                    knight: Bitboard(148618787703226432),
                    bishop: Bitboard(288230376151711780),
                    rook: Bitboard(4683743612465315969),
                    queen: Bitboard(37154696925806592),
                    king: Bitboard(1152921504606846984),
                },
                ByColor {
                    black: Bitboard(6284509431698817024),
                    white: Bitboard(36028865738826989),
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
            castling_rights: Bitboard(72057594037927936),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
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
