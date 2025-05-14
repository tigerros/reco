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

/// Italian Game: Scotch Gambit.
pub const SCOTCH_GAMBIT: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<56>(),
    },
    name: "Italian Game",
    variation: &["Scotch Gambit"],
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
            role: Pawn,
            from: D2,
            capture: None,
            to: D4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: E5,
            capture: Some(Pawn),
            to: D4,
            promotion: None,
        },
        Castle { king: E1, rook: H1 },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272519836559104),
                knight: Bitboard(39582420697090),
                bishop: Bitboard(2594073385432514564),
                rook: Bitboard(9295429630892703777),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606847040),
            },
            ByColor {
                black: Bitboard(13686197375155044352),
                white: Bitboard(337700719),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703744),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
pub mod max_lange_attack_accepted;
pub use max_lange_attack_accepted::MAX_LANGE_ATTACK_ACCEPTED;
pub mod janowski_defense;
pub use janowski_defense::JANOWSKI_DEFENSE;
pub mod de_riviere_defense;
pub use de_riviere_defense::DE_RIVIERE_DEFENSE;
pub mod double_gambit_accepted;
pub use double_gambit_accepted::DOUBLE_GAMBIT_ACCEPTED;
pub mod walbrodt_baird_gambit;
pub use walbrodt_baird_gambit::WALBRODT_BAIRD_GAMBIT;
pub mod canal_variation;
pub use canal_variation::CANAL_VARIATION;
pub mod anderssen_attack;
pub use anderssen_attack::ANDERSSEN_ATTACK;
pub mod nakhmanson_gambit;
pub use nakhmanson_gambit::NAKHMANSON_GAMBIT;
pub mod max_lange_attack;
pub use max_lange_attack::MAX_LANGE_ATTACK;
