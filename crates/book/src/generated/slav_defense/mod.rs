use core::num::NonZeroU32;
use core::panic;
use deranged::RangedU8;
use reco_core::{Code, Opening, Volume};
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

/// Slav Defense.
pub const SLAV_DEFENSE: [Opening<'static, &str>; 3] = [
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<10>(),
        },
        name: "Slav Defense",
        variation: &[],
        moves: &[
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
                from: C2,
                capture: None,
                to: C4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C7,
                capture: None,
                to: C6,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(68402851948327680),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18443089331418562560),
                    white: Bitboard(201389055),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<10>(),
        },
        name: "Slav Defense",
        variation: &[],
        moves: &[
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
                from: C2,
                capture: None,
                to: C4,
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
                role: Knight,
                from: B1,
                capture: None,
                to: C3,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(68402851948327680),
                    knight: Bitboard(4755801206503505984),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18443089331418562560),
                    white: Bitboard(201651197),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<10>(),
        },
        name: "Slav Defense",
        variation: &[],
        moves: &[
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
                from: C2,
                capture: None,
                to: C4,
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
                role: Knight,
                from: B1,
                capture: None,
                to: C3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D5,
                capture: Some(Pawn),
                to: C4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(68402817588589312),
                    knight: Bitboard(4755801206503505984),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18443089297125933056),
                    white: Bitboard(134542333),
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
        },
    },
];
pub mod two_knights_attack;
pub use two_knights_attack::TWO_KNIGHTS_ATTACK;
pub mod schlechter_variation;
pub use schlechter_variation::SCHLECHTER_VARIATION;
pub mod bonet_gambit;
pub use bonet_gambit::BONET_GAMBIT;
pub mod alekhine_variation;
pub use alekhine_variation::ALEKHINE_VARIATION;
pub mod smyslov_variation;
pub use smyslov_variation::SMYSLOV_VARIATION;
pub mod suchting_variation;
pub use suchting_variation::SUCHTING_VARIATION;
pub mod winawer_countergambit;
pub use winawer_countergambit::WINAWER_COUNTERGAMBIT;
pub mod chebanenko_variation;
pub use chebanenko_variation::CHEBANENKO_VARIATION;
pub mod breyer_variation;
pub use breyer_variation::BREYER_VARIATION;
pub mod slav_gambit;
pub mod steiner_variation;
pub use steiner_variation::STEINER_VARIATION;
pub mod modern_line;
pub use modern_line::MODERN_LINE;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod alapin_variation;
pub use alapin_variation::ALAPIN_VARIATION;
pub mod czech_variation;
pub use czech_variation::CZECH_VARIATION;
pub mod quiet_variation;
pub use quiet_variation::QUIET_VARIATION;
pub mod diemer_gambit;
pub use diemer_gambit::DIEMER_GAMBIT;
pub mod soultanbeieff_variation;
pub use soultanbeieff_variation::SOULTANBEIEFF_VARIATION;
pub mod three_knights_variation;
pub use three_knights_variation::THREE_KNIGHTS_VARIATION;
pub mod geller_gambit;
pub use geller_gambit::GELLER_GAMBIT;
