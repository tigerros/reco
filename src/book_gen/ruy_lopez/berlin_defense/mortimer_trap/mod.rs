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
# use reco::book::ruy_lopez::berlin_defense::MORTIMER_TRAP;
assert_eq!(MORTIMER_TRAP.original_name(), "Ruy Lopez: Berlin Defense, Mortimer Trap");
```"#
)]
pub static MORTIMER_TRAP: Variation = Variation {
    name: "Mortimer Trap",
    parent: Some(&super::BERLIN_DEFENSE),
    variations: &[],
    lines: &[Line {
        parent: &MORTIMER_TRAP,
        code: Code {
            volume: Volume::C,
            category: Category(RangedU8::new_static::<6>()),
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
                role: Bishop,
                from: Square::F1,
                capture: None,
                to: Square::B5,
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
                from: Square::D2,
                capture: None,
                to: Square::D3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::C6,
                capture: None,
                to: Square::E7,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::F3,
                capture: Some(Pawn),
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C7,
                capture: None,
                to: Square::C6,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(66151017842534144),
                    knight: Bitboard(4538852718936066),
                    bishop: Bitboard(2594073393955340292),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13689575074741354496),
                    white: Bitboard(77578430367),
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
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
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
