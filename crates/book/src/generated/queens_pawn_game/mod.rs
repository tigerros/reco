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

/// Queen's Pawn Game.
pub const QUEENS_PAWN_GAME: [Opening<'static, &str>; 5] = [
    Opening {
        code: Code {
            volume: Volume::A,
            category: RangedU8::new_static::<40>(),
        },
        name: "Queen's Pawn Game",
        variation: &[],
        moves: &[Normal {
            role: Pawn,
            from: D2,
            capture: None,
            to: D4,
            promotion: None,
        }],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(71776119195498240),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18446462598732840960),
                    white: Bitboard(134281215),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(1) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::A,
            category: RangedU8::new_static::<41>(),
        },
        name: "Queen's Pawn Game",
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
                to: D6,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(69533115474835200),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18444219595012177920),
                    white: Bitboard(134281215),
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
        },
    },
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<0>(),
        },
        name: "Queen's Pawn Game",
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
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(69524353741551360),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18444210833278894080),
                    white: Bitboard(134281215),
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
        },
    },
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<0>(),
        },
        name: "Queen's Pawn Game",
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
                from: E2,
                capture: None,
                to: E3,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(69524353742595840),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18444210833278894080),
                    white: Bitboard(135325695),
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
        },
    },
    Opening {
        code: Code {
            volume: Volume::D,
            category: RangedU8::new_static::<0>(),
        },
        name: "Queen's Pawn Game",
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
                from: E2,
                capture: None,
                to: E3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(69524353742595840),
                    knight: Bitboard(144150372447944770),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13832559999223595008),
                    white: Bitboard(135325695),
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
        },
    },
];
pub mod hubsch_gambit;
pub use hubsch_gambit::HUBSCH_GAMBIT;
pub mod liedmann_gambit;
pub use liedmann_gambit::LIEDMANN_GAMBIT;
pub mod accelerated_london_system;
pub mod veresov_attack;
pub use accelerated_london_system::ACCELERATED_LONDON_SYSTEM;
pub mod london_system;
pub use london_system::LONDON_SYSTEM;
pub mod chandler_gambit;
pub use chandler_gambit::CHANDLER_GAMBIT;
pub mod chigorin_variation;
pub mod veresov;
pub use chigorin_variation::CHIGORIN_VARIATION;
pub mod zurich_gambit;
pub use zurich_gambit::ZURICH_GAMBIT;
pub mod krause_variation;
pub use krause_variation::KRAUSE_VARIATION;
pub mod colle_system;
pub use colle_system::COLLE_SYSTEM;
pub mod zukertort_variation;
pub use zukertort_variation::ZUKERTORT_VARIATION;
pub mod torre_attack;
pub use torre_attack::TORRE_ATTACK;
pub mod barry_attack;
pub use barry_attack::BARRY_ATTACK;
pub mod anti_torre;
pub use anti_torre::ANTI_TORRE;
pub mod symmetrical_variation;
pub use symmetrical_variation::SYMMETRICAL_VARIATION;
pub mod levitsky_attack;
pub use levitsky_attack::LEVITSKY_ATTACK;
pub mod stonewall_attack;
pub use stonewall_attack::STONEWALL_ATTACK;
pub mod modern_defense;
pub use modern_defense::MODERN_DEFENSE;
pub mod mason_attack;
pub use mason_attack::MASON_ATTACK;
pub mod anglo_slav_opening;
pub use anglo_slav_opening::ANGLO_SLAV_OPENING;
pub mod zilbermints_countergambit;
pub use zilbermints_countergambit::ZILBERMINTS_COUNTERGAMBIT;
