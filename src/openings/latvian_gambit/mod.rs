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

/// Latvian Gambit.
pub const LATVIAN_GAMBIT: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        subcategory: RangedU8::new_static::<40>(),
    },
    name: "Latvian Gambit",
    variation: &[],
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
            role: Pawn,
            from: F7,
            capture: None,
            to: F5,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(58265526606032640),
                knight: Bitboard(4755801206505340930),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(18432952006009159680),
                white: Bitboard(270593983),
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
}];
pub mod corkscrew_gambit;
pub use corkscrew_gambit::CORKSCREW_GAMBIT;
pub mod senechaud_gambit;
pub use senechaud_gambit::SENECHAUD_GAMBIT;
pub mod clam_gambit;
pub use clam_gambit::CLAM_GAMBIT;
pub mod mason_countergambit;
pub use mason_countergambit::MASON_COUNTERGAMBIT;
pub mod lobster_gambit;
pub use lobster_gambit::LOBSTER_GAMBIT;
pub mod mlotkowski_variation;
pub use mlotkowski_variation::MLOTKOWSKI_VARIATION;
pub mod fraser_defense;
pub use fraser_defense::FRASER_DEFENSE;
pub mod diepstraten_countergambit;
pub use diepstraten_countergambit::DIEPSTRATEN_COUNTERGAMBIT;
pub mod behting_variation;
pub use behting_variation::BEHTING_VARIATION;
pub mod mayet_attack;
pub use mayet_attack::MAYET_ATTACK;
pub mod corkscrew_countergambit;
pub use corkscrew_countergambit::CORKSCREW_COUNTERGAMBIT;
pub mod greco_variation;
pub use greco_variation::GRECO_VARIATION;
