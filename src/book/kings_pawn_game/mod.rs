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
/// King's Pawn Game.
pub static KINGS_PAWN_GAME: [Opening<&str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<0>(),
        },
        name: Cow::Borrowed(&["King's Pawn Game"]),
        moves: Cow::Borrowed(&[Normal {
            role: Pawn,
            from: E2,
            capture: None,
            to: E4,
            promotion: None,
        }]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(71776119329713920),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18446462598732840960),
                    white: Bitboard(268496895),
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
        }),
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<20>(),
        },
        name: Cow::Borrowed(&["King's Pawn Game"]),
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
                from: E7,
                capture: None,
                to: E5,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272588421820160),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18441959067824947200),
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
pub mod alapin_opening;
pub use alapin_opening::ALAPIN_OPENING;
pub mod bavarian_gambit;
pub use bavarian_gambit::BAVARIAN_GAMBIT;
pub mod beyer_gambit;
pub use beyer_gambit::BEYER_GAMBIT;
pub mod busch_gass_gambit;
pub use busch_gass_gambit::BUSCH_GASS_GAMBIT;
pub mod clam_variation;
pub mod damiano_defense;
pub use damiano_defense::DAMIANO_DEFENSE;
pub mod dresden_opening;
pub use dresden_opening::DRESDEN_OPENING;
pub mod gunderam_defense;
pub mod gunderam_gambit;
pub use gunderam_gambit::GUNDERAM_GAMBIT;
pub mod kings_head_opening;
pub use kings_head_opening::KINGS_HEAD_OPENING;
pub mod la_bourdonnais_gambit;
pub use la_bourdonnais_gambit::LA_BOURDONNAIS_GAMBIT;
pub mod leonardis_variation;
pub use leonardis_variation::LEONARDIS_VARIATION;
pub mod mac_leod_attack;
pub use mac_leod_attack::MAC_LEOD_ATTACK;
pub mod maroczy_defense;
pub use maroczy_defense::MAROCZY_DEFENSE;
pub mod mc_connell_defense;
pub use mc_connell_defense::MC_CONNELL_DEFENSE;
pub mod mengarinis_opening;
pub use mengarinis_opening::MENGARINIS_OPENING;
pub mod napoleon_attack;
pub use napoleon_attack::NAPOLEON_ATTACK;
pub mod pachman_wing_gambit;
pub use pachman_wing_gambit::PACHMAN_WING_GAMBIT;
pub mod philidor_gambit;
pub use philidor_gambit::PHILIDOR_GAMBIT;
pub mod schulze_muller_gambit;
pub use schulze_muller_gambit::SCHULZE_MULLER_GAMBIT;
pub mod tayler_opening;
pub use tayler_opening::TAYLER_OPENING;
pub mod tortoise_opening;
pub use tortoise_opening::TORTOISE_OPENING;
pub mod wayward_queen_attack;
pub use wayward_queen_attack::WAYWARD_QUEEN_ATTACK;
pub mod weber_gambit;
pub use weber_gambit::WEBER_GAMBIT;
