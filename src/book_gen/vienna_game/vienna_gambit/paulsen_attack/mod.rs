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
# use reco::book::vienna_game::vienna_gambit::PAULSEN_ATTACK;
assert_eq!(PAULSEN_ATTACK.original_name(), "Vienna Game: Vienna Gambit, Paulsen Attack");
```"#
)]
pub static PAULSEN_ATTACK: Variation = Variation {
    name: "Paulsen Attack",
    parent: Some(&super::VIENNA_GAMBIT),
    variations: &[],
    lines: &[Line {
        parent: &PAULSEN_ATTACK,
        code: Code {
            volume: Volume::C,
            category: Category(RangedU8::new_static::<2>()),
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
                from: Square::G8,
                capture: None,
                to: Square::F6,
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
                from: Square::D7,
                capture: None,
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::F4,
                capture: Some(Pawn),
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::F6,
                capture: Some(Pawn),
                to: Square::E4,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D1,
                capture: None,
                to: Square::F3,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(65020822699429632),
                    knight: Bitboard(144115188344553536),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752305520640),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13828021215492571136),
                    white: Bitboard(68721889269),
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
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
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
