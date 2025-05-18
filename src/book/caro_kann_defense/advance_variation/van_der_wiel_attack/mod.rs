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
/// Caro-Kann Defense: Advance Variation, Van der Wiel Attack.
pub static VAN_DER_WIEL_ATTACK: [Opening<&str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<12>(),
        },
        name: Cow::Borrowed(&[
            "Caro-Kann Defense",
            "Advance Variation",
            "Van der Wiel Attack",
        ]),
        moves: Cow::Borrowed(&[
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E4,
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
                from: D2,
                capture: None,
                to: D4,
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
                capture: None,
                to: E5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C8,
                capture: None,
                to: F5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: B1,
                capture: None,
                to: C3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E7,
                capture: None,
                to: E6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: G2,
                capture: None,
                to: G4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F5,
                capture: None,
                to: G6,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G1,
                capture: None,
                to: E2,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C6,
                capture: None,
                to: C5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: H2,
                capture: None,
                to: H4,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(63912535513900800),
                    knight: Bitboard(4755801206503510016),
                    bishop: Bitboard(2305913377957871652),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18150438935703060480),
                    white: Bitboard(72075196349),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<12>(),
        },
        name: Cow::Borrowed(&[
            "Caro-Kann Defense",
            "Advance Variation",
            "Van der Wiel Attack",
        ]),
        moves: Cow::Borrowed(&[
            Normal {
                role: Pawn,
                from: E2,
                capture: None,
                to: E4,
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
                from: D2,
                capture: None,
                to: D4,
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
                capture: None,
                to: E5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C8,
                capture: None,
                to: F5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: B1,
                capture: None,
                to: C3,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(68402920600692480),
                    knight: Bitboard(4755801206503505984),
                    bishop: Bitboard(2305843146652647460),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18154859092705804288),
                    white: Bitboard(68854015997),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 2,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
];
pub mod dreyev_defense;
pub use dreyev_defense::DREYEV_DEFENSE;
