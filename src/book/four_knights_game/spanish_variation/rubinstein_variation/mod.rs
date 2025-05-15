use crate::{Code, Opening, Volume};
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

#[allow(clippy::doc_markdown)]
/// Four Knights Game: Spanish Variation, Rubinstein Variation.
pub const RUBINSTEIN_VARIATION: [Opening<'static, &str>; 3] = [
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<48>(),
        },
        name: "Four Knights Game",
        variation: &["Spanish Variation", "Rubinstein Variation"],
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
                role: Knight,
                from: B1,
                capture: None,
                to: C3,
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
                role: Bishop,
                from: F1,
                capture: None,
                to: B5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: C6,
                capture: None,
                to: D4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: B5,
                capture: None,
                to: E2,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272588421820160),
                    knight: Bitboard(35184508665856),
                    bishop: Bitboard(2594073385365409796),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13686193045828009984),
                    white: Bitboard(270860189),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 7,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<48>(),
        },
        name: "Four Knights Game",
        variation: &["Spanish Variation", "Rubinstein Variation"],
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
                role: Knight,
                from: B1,
                capture: None,
                to: C3,
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
                role: Bishop,
                from: F1,
                capture: None,
                to: B5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: C6,
                capture: None,
                to: D4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272588421820160),
                    knight: Bitboard(35184508665856),
                    bishop: Bitboard(2594073393955340292),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13686193045828009984),
                    white: Bitboard(8860790685),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 6,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<48>(),
        },
        name: "Four Knights Game",
        variation: &["Spanish Variation", "Rubinstein Variation"],
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
                from: G8,
                capture: None,
                to: F6,
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
                to: B5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: C6,
                capture: None,
                to: D4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: Some(Pawn),
                to: E5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: F2,
                capture: None,
                to: F4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272520239206144),
                    knight: Bitboard(35253226045440),
                    bishop: Bitboard(2594073393955340292),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(4503599627370504),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13114235824432480256),
                    white: Bitboard(78115032989),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
];
pub mod henneberger_variation;
pub use henneberger_variation::HENNEBERGER_VARIATION;
pub mod maroczy_variation;
pub use maroczy_variation::MAROCZY_VARIATION;
pub mod marshall_gambit;
pub use marshall_gambit::MARSHALL_GAMBIT;
pub mod marshall_variation;
pub use marshall_variation::MARSHALL_VARIATION;
