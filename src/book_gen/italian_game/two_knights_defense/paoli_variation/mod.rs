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
    clippy::enum_glob_use,
    reason = "there's 64 variants in this enum, importing them all is stupid"
)]
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square::*;
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
# use reco::book::italian_game::two_knights_defense::PAOLI_VARIATION;
assert_eq!(PAOLI_VARIATION.original_name(), "Italian Game: Two Knights Defense, Paoli Variation");
```"#
)]
pub static PAOLI_VARIATION: Variation = Variation {
    name: "Paoli Variation",
    parent: Some(&super::TWO_KNIGHTS_DEFENSE),
    variations: &[],
    lines: &[Line {
        parent: &PAOLI_VARIATION,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<58>(),
        },
        moves: &[
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E7,
                capture: None,
                to: E5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G1,
                capture: None,
                to: F3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: B8,
                capture: None,
                to: C6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F1,
                capture: None,
                to: C4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: None,
                to: G5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D7,
                capture: None,
                to: D5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E4,
                capture: Some(Pawn),
                to: D5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: C6,
                capture: None,
                to: A5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C4,
                capture: None,
                to: B5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C7,
                capture: None,
                to: C6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D5,
                capture: Some(Pawn),
                to: C6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: B7,
                capture: Some(Pawn),
                to: C6,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D1,
                capture: None,
                to: F3,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D8,
                capture: None,
                to: C7,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: B5,
                capture: None,
                to: D3,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(63336336525946624),
                    knight: Bitboard(35463544963074),
                    bishop: Bitboard(2594073385365929988),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(1125899908939776),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13106921945964740608),
                    white: Bitboard(274880589719),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
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
