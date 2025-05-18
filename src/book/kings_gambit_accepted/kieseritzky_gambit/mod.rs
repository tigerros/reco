#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByRole, ByColor, Setup};
use core::num::NonZeroU32;
use crate::{Opening, Code, Volume};
use alloc::borrow::Cow;
use deranged::RangedU8;
use core::panic;

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// King's Gambit Accepted: Kieseritzky Gambit.
pub static KIESERITZKY_GAMBIT: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<39>(),
    },
    name: Cow::Borrowed(&["King's Gambit Accepted", "Kieseritzky Gambit"]),
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
    Normal {
        role: Pawn,
        from: F2,
        capture: None,
        to: F4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E5,
        capture: Some(
            Pawn,
        ),
        to: F4,
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
        from: G7,
        capture: None,
        to: G5,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: H2,
        capture: None,
        to: H4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: G5,
        capture: None,
        to: G4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F3,
        capture: None,
        to: E5,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(49258124950916864),
                knight: Bitboard(4755801275222720514),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18423944602206601216),
                white: Bitboard(71135416255)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];pub mod anderssen_defense;
pub use anderssen_defense::ANDERSSEN_DEFENSE;
pub mod anderssen_cordel_gambit;
pub use anderssen_cordel_gambit::ANDERSSEN_CORDEL_GAMBIT;
pub mod berlin_defense;
pub use berlin_defense::BERLIN_DEFENSE;
pub mod brentano_defense;
pub use brentano_defense::BRENTANO_DEFENSE;
pub mod cotter_gambit;
pub use cotter_gambit::COTTER_GAMBIT;
pub mod kolisch_defense;
pub use kolisch_defense::KOLISCH_DEFENSE;
pub mod long_whip;
pub use long_whip::LONG_WHIP;
pub mod neumann_defense;
pub use neumann_defense::NEUMANN_DEFENSE;
pub mod paulsen_defense;
pub use paulsen_defense::PAULSEN_DEFENSE;
pub mod paulsen_defense_deferred;
pub use paulsen_defense_deferred::PAULSEN_DEFENSE_DEFERRED;
pub mod rice_gambit;
pub use rice_gambit::RICE_GAMBIT;
pub mod rosenthal_defense;
pub use rosenthal_defense::ROSENTHAL_DEFENSE;
