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
# use reco::book::caro_kann_defense::hillbilly_attack::SCHAEFFER_GAMBIT;
assert_eq!(SCHAEFFER_GAMBIT.original_name(), "Caro-Kann Defense: Hillbilly Attack, Schaeffer Gambit");
```"#
)]
pub static SCHAEFFER_GAMBIT: Variation = Variation {
    name: "Schaeffer Gambit",
    parent: Some(&super::HILLBILLY_ATTACK),
    variations: &[],
    lines: &[Line {
        parent: &SCHAEFFER_GAMBIT,
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
                role: Bishop,
                from: Square::F1,
                capture: None,
                to: Square::C4,
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
                role: Bishop,
                from: Square::C4,
                capture: None,
                to: Square::B3,
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
                role: Queen,
                from: Square::D1,
                capture: None,
                to: Square::H5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(68402817655697152),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365536772),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576461302059237376),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18443089297327259648),
                    white: Bitboard(549756006359),
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
            halfmoves: 1,
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
