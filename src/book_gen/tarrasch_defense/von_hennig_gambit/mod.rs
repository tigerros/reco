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
# use reco::book::tarrasch_defense::VON_HENNIG_GAMBIT;
assert_eq!(VON_HENNIG_GAMBIT.original_name(), "Tarrasch Defense: von Hennig Gambit");
```"#
)]
pub static VON_HENNIG_GAMBIT: Variation = Variation {
    name: "von Hennig Gambit",
    parent: Some(&super::TARRASCH_DEFENSE),
    variations: &[],
    lines: &[Line {
        parent: &VON_HENNIG_GAMBIT,
        code: Code {
            volume: Volume::D,
            category: Category(RangedU8::new_static::<3>()),
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
                role: Pawn,
                from: Square::C7,
                capture: None,
                to: Square::C5,
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
                role: Pawn,
                from: Square::C5,
                capture: Some(Pawn),
                to: Square::D4,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D1,
                capture: Some(Pawn),
                to: Square::D4,
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
                role: Queen,
                from: Square::D4,
                capture: None,
                to: Square::D1,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E6,
                capture: Some(Pawn),
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D1,
                capture: Some(Pawn),
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C8,
                capture: None,
                to: Square::E6,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(63894819713381120),
                    knight: Bitboard(4611690416474161216),
                    bishop: Bitboard(2305860601399738404),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460786663161856),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18006257725389930496),
                    white: Bitboard(34360062965),
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
            halfmoves: 1,
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
