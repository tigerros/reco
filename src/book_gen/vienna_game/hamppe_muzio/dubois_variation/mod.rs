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
# use reco::book::vienna_game::hamppe_muzio::DUBOIS_VARIATION;
assert_eq!(DUBOIS_VARIATION.original_name(), "Vienna Game: Hamppe-Muzio, Dubois Variation");
```"#
)]
pub static DUBOIS_VARIATION: Variation = Variation {
    name: "Dubois Variation",
    parent: Some(&super::HAMPPE_MUZIO),
    variations: &[],
    lines: &[Line {
        parent: &DUBOIS_VARIATION,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<2>(),
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
                to: Square::E5,
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
                from: Square::B8,
                capture: None,
                to: Square::C6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::F2,
                capture: None,
                to: Square::F4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E5,
                capture: Some(Pawn),
                to: Square::F4,
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
                from: Square::G7,
                capture: None,
                to: Square::G5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F1,
                capture: None,
                to: Square::C4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::G5,
                capture: None,
                to: Square::G4,
                promotion: None,
            },
            Castle {
                king: Square::E1,
                rook: Square::H1,
            },
            Normal {
                role: Pawn,
                from: Square::G4,
                capture: Some(Knight),
                to: Square::F3,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D1,
                capture: Some(Pawn),
                to: Square::F3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::C6,
                capture: None,
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::F3,
                capture: Some(Pawn),
                to: Square::F4,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D8,
                capture: None,
                to: Square::F6,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(49258121192853248),
                    knight: Bitboard(4611686087147126784),
                    bishop: Bitboard(2594073385432514564),
                    rook: Bitboard(9295429630892703777),
                    queen: Bitboard(35184908959744),
                    king: Bitboard(1152921504606847040),
                },
                ByColor {
                    black: Bitboard(17703403913308274688),
                    white: Bitboard(872730469),
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
            castling_rights: Bitboard(9295429630892703744),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
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
