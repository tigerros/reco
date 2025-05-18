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
/// Modern Defense.
pub static MODERN_DEFENSE: [Opening<&str>; 3] = [
    Opening {
        code: Code {
            volume: Volume::A,
            category: RangedU8::new_static::<41>(),
        },
        name: Cow::Borrowed(&["Modern Defense"]),
        moves: Cow::Borrowed(&[
            Normal {
                role: Pawn,
                from: D2,
                capture: None,
                to: D4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: G7,
                capture: None,
                to: G6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C2,
                capture: None,
                to: C4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: G7,
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
                from: D7,
                capture: None,
                to: D6,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(51589085776638720),
                    knight: Bitboard(4755801206503505984),
                    bishop: Bitboard(306244774661193764),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(16138446954542661632),
                    white: Bitboard(201651197),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<6>(),
        },
        name: Cow::Borrowed(&["Modern Defense"]),
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
                from: G7,
                capture: None,
                to: G6,
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
                role: Bishop,
                from: F8,
                capture: None,
                to: G7,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(53832089698625280),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(306244774661193764),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(16140689958263324672),
                    white: Bitboard(402712575),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<6>(),
        },
        name: Cow::Borrowed(&["Modern Defense"]),
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
                from: G7,
                capture: None,
                to: G6,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(53832089564409600),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18428518568967536640),
                    white: Bitboard(268496895),
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
];
pub mod anti_modern;
pub use anti_modern::ANTI_MODERN;
pub mod averbakh_system;
pub use averbakh_system::AVERBAKH_SYSTEM;
pub mod averbakh_variation;
pub mod beefeater_variation;
pub use beefeater_variation::BEEFEATER_VARIATION;
pub mod bishop_attack;
pub use bishop_attack::BISHOP_ATTACK;
pub mod dunworthy_variation;
pub use dunworthy_variation::DUNWORTHY_VARIATION;
pub mod fianchetto_gambit;
pub use fianchetto_gambit::FIANCHETTO_GAMBIT;
pub mod gellers_system;
pub use gellers_system::GELLERS_SYSTEM;
pub mod gurgenidze_defense;
pub use gurgenidze_defense::GURGENIDZE_DEFENSE;
pub mod kotov_variation;
pub use kotov_variation::KOTOV_VARIATION;
pub mod lizard_defense;
pub mod masur_gambit;
pub use masur_gambit::MASUR_GAMBIT;
pub mod modern_pterodactyl;
pub use modern_pterodactyl::MODERN_PTERODACTYL;
pub mod mongredien_defense;
pub mod neo_modern_defense;
pub use neo_modern_defense::NEO_MODERN_DEFENSE;
pub mod norwegian_defense;
pub use norwegian_defense::NORWEGIAN_DEFENSE;
pub mod pseudo_austrian_attack;
pub use pseudo_austrian_attack::PSEUDO_AUSTRIAN_ATTACK;
pub mod pterodactyl_variation;
pub use pterodactyl_variation::PTERODACTYL_VARIATION;
pub mod randspringer_variation;
pub use randspringer_variation::RANDSPRINGER_VARIATION;
pub mod semi_averbakh_variation;
pub mod standard_defense;
pub use standard_defense::STANDARD_DEFENSE;
pub mod standard_line;
pub use standard_line::STANDARD_LINE;
pub mod three_pawns_attack;
pub use three_pawns_attack::THREE_PAWNS_ATTACK;
pub mod two_knights_variation;
pub use two_knights_variation::TWO_KNIGHTS_VARIATION;
pub mod westermann_gambit;
pub use westermann_gambit::WESTERMANN_GAMBIT;
pub mod wind_gambit;
pub use wind_gambit::WIND_GAMBIT;
