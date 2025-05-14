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

/// King's Indian Defense.
pub const KINGS_INDIAN_DEFENSE: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::E,
        category: RangedU8::new_static::<61>(),
    },
    name: "King's Indian Defense",
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
            role: Knight,
            from: G8,
            capture: None,
            to: F6,
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
            from: G7,
            capture: None,
            to: G6,
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
                pawn: Bitboard(53832089497301760),
                knight: Bitboard(144150372448206912),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(13816867734912237568),
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
}];
pub mod accelerated_averbakh_variation;
pub use accelerated_averbakh_variation::ACCELERATED_AVERBAKH_VARIATION;
pub mod averbakh_variation;
pub use averbakh_variation::AVERBAKH_VARIATION;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod fianchetto_variation;
pub mod four_pawns_attack;
pub use four_pawns_attack::FOUR_PAWNS_ATTACK;
pub mod kazakh_variation;
pub use kazakh_variation::KAZAKH_VARIATION;
pub mod kramer_variation;
pub use kramer_variation::KRAMER_VARIATION;
pub mod larsen_variation;
pub use larsen_variation::LARSEN_VARIATION;
pub mod makogonov_variation;
pub use makogonov_variation::MAKOGONOV_VARIATION;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod orthodox_variation;
pub use orthodox_variation::ORTHODOX_VARIATION;
pub mod petrosian_variation;
pub use petrosian_variation::PETROSIAN_VARIATION;
pub mod pomar_system;
pub use pomar_system::POMAR_SYSTEM;
pub mod samisch_variation;
pub use samisch_variation::SAMISCH_VARIATION;
pub mod santasiere_variation;
pub use santasiere_variation::SANTASIERE_VARIATION;
pub mod semi_averbakh_system;
pub use semi_averbakh_system::SEMI_AVERBAKH_SYSTEM;
pub mod semi_classical_variation;
pub use semi_classical_variation::SEMI_CLASSICAL_VARIATION;
pub mod six_pawns_attack;
pub use six_pawns_attack::SIX_PAWNS_ATTACK;
pub mod smyslov_variation;
pub use smyslov_variation::SMYSLOV_VARIATION;
pub mod steiner_attack;
pub use steiner_attack::STEINER_ATTACK;
pub mod zinnowitz_variation;
pub use zinnowitz_variation::ZINNOWITZ_VARIATION;
