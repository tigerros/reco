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
# use reco::book::kings_gambit_accepted::DOUBLE_MUZIO_GAMBIT;
assert_eq!(DOUBLE_MUZIO_GAMBIT.original_name(), "King's Gambit Accepted: Double Muzio Gambit");
```"#
)]
pub static DOUBLE_MUZIO_GAMBIT: Variation = Variation {
    name: "Double Muzio Gambit",
    parent: Some(&super::KINGS_GAMBIT_ACCEPTED),
    variations: &[
        &BALDWIN_GAMBIT,
        &BELLO_GAMBIT,
        &PAULSEN_DEFENSE,
        &YOUNG_GAMBIT,
    ],
    lines: &[Line {
        parent: &DOUBLE_MUZIO_GAMBIT,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<3>(),
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
                role: Queen,
                from: Square::D8,
                capture: None,
                to: Square::F6,
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
                role: Queen,
                from: Square::F6,
                capture: Some(Pawn),
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C4,
                capture: Some(Pawn),
                to: Square::F7,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(40250922206547712),
                    knight: Bitboard(4755801206503243778),
                    bishop: Bitboard(2603080584620146692),
                    rook: Bitboard(9295429630892703777),
                    queen: Bitboard(68721573888),
                    king: Bitboard(1152921504606847040),
                },
                ByColor {
                    black: Bitboard(17838476718294171648),
                    white: Bitboard(9007199256891239),
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
            castling_rights: Bitboard(9295429630892703744),
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
pub mod baldwin_gambit;
pub use baldwin_gambit::BALDWIN_GAMBIT;
pub mod bello_gambit;
pub use bello_gambit::BELLO_GAMBIT;
pub mod paulsen_defense;
pub use paulsen_defense::PAULSEN_DEFENSE;
pub mod young_gambit;
pub use young_gambit::YOUNG_GAMBIT;
