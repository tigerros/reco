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
# use reco::book::queens_gambit_declined::tartakower_defense::MAKOGONOV_EXCHANGE_VARIATION;
assert_eq!(MAKOGONOV_EXCHANGE_VARIATION.original_name(), "Queen's Gambit Declined: Tartakower Defense, Makogonov Exchange Variation");
```"#
)]
pub static MAKOGONOV_EXCHANGE_VARIATION: Variation = Variation {
    name: "Makogonov Exchange Variation",
    parent: Some(&super::TARTAKOWER_DEFENSE),
    variations: &[],
    lines: &[Line {
        parent: &MAKOGONOV_EXCHANGE_VARIATION,
        code: Code {
            volume: Volume::D,
            category: Category(RangedU8::new_static::<5>()),
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
                to: Square::E6,
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
                role: Knight,
                from: Square::G1,
                capture: None,
                to: Square::F3,
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
                role: Pawn,
                from: Square::H7,
                capture: None,
                to: Square::H6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::G5,
                capture: None,
                to: Square::H4,
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
                role: Pawn,
                from: Square::B7,
                capture: None,
                to: Square::B6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C4,
                capture: Some(Pawn),
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::F6,
                capture: Some(Pawn),
                to: Square::D5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(28589501480755968),
                    knight: Bitboard(144115222437953536),
                    bishop: Bitboard(292733977926565920),
                    rook: Bitboard(2377900603251622017),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387920),
                },
                ByColor {
                    black: Bitboard(8031486073542541312),
                    white: Bitboard(2285167545),
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
            halfmoves: 0,
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
