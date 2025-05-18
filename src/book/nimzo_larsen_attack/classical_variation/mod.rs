use crate::{Code, Opening, Volume};
use alloc::borrow::Cow;
use core::num::NonZeroU32;
use core::panic;
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
    clippy::enum_glob_use,
    reason = "there's 64 variants in this enum, importing them all is stupid"
)]
use shakmaty::Square::*;
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByColor, ByRole, Setup};

#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// Nimzo-Larsen Attack: Classical Variation.
pub static CLASSICAL_VARIATION: [Opening<&str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::A,
            category: RangedU8::new_static::<1>(),
        },
        name: Cow::Borrowed(&["Nimzo-Larsen Attack", "Classical Variation"]),
        moves: Cow::Borrowed(&[
            Normal {
                role: Pawn,
                from: B2,
                capture: None,
                to: B3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D7,
                capture: None,
                to: D5,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(69524353607466240),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18444210833278894080),
                    white: Bitboard(196095),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::A,
            category: RangedU8::new_static::<6>(),
        },
        name: Cow::Borrowed(&["Nimzo-Larsen Attack", "Classical Variation"]),
        moves: Cow::Borrowed(&[
            Normal {
                role: Knight,
                from: G1,
                capture: None,
                to: F3,
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
                from: B2,
                capture: None,
                to: B3,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(69524353607466240),
                    knight: Bitboard(4755801206505340930),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18444210833278894080),
                    white: Bitboard(2293183),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
];
