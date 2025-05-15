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

/// Nimzo-Larsen Attack.
pub const NIMZO_LARSEN_ATTACK: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::A,
        category: RangedU8::new_static::<1>(),
    },
    name: "Nimzo-Larsen Attack",
    variation: &[],
    moves: &[Normal {
        role: Pawn,
        from: B2,
        capture: None,
        to: B3,
        promotion: None,
    }],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(71776119061413120),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(18446462598732840960),
                white: Bitboard(196095),
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
}];
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod dutch_variation;
pub use dutch_variation::DUTCH_VARIATION;
pub mod english_variation;
pub use english_variation::ENGLISH_VARIATION;
pub mod graz_attack;
pub use graz_attack::GRAZ_ATTACK;
pub mod indian_variation;
pub use indian_variation::INDIAN_VARIATION;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod norfolk_gambit;
pub use norfolk_gambit::NORFOLK_GAMBIT;
pub mod pachman_gambit;
pub use pachman_gambit::PACHMAN_GAMBIT;
pub mod polish_variation;
pub use polish_variation::POLISH_VARIATION;
pub mod ringelbach_gambit;
pub use ringelbach_gambit::RINGELBACH_GAMBIT;
pub mod spike_variation;
pub use spike_variation::SPIKE_VARIATION;
pub mod symmetrical_variation;
pub use symmetrical_variation::SYMMETRICAL_VARIATION;
