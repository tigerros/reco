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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Queen's Indian Defense.
pub static QUEENS_INDIAN_DEFENSE: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::E,
        category: RangedU8::new_static::<12>(),
    },
    name: Cow::Borrowed(&["Queen's Indian Defense"]),
    moves: Cow::Borrowed(&[
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
        from: E7,
        capture: None,
        to: E6,
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
        from: B7,
        capture: None,
        to: B6,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(66729360891114240),
                knight: Bitboard(144150372450041858),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13829765006306050048),
                white: Bitboard(203486143)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];pub mod anti_queens_indian_system;
pub use anti_queens_indian_system::ANTI_QUEENS_INDIAN_SYSTEM;
pub mod averbakh_variation;
pub use averbakh_variation::AVERBAKH_VARIATION;
pub mod buerger_variation;
pub use buerger_variation::BUERGER_VARIATION;
pub mod capablanca_variation;
pub use capablanca_variation::CAPABLANCA_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod euwe_variation;
pub use euwe_variation::EUWE_VARIATION;
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
pub mod kasparov_variation;
pub use kasparov_variation::KASPAROV_VARIATION;
pub mod kasparov_petrosian_variation;
pub use kasparov_petrosian_variation::KASPAROV_PETROSIAN_VARIATION;
pub mod miles_variation;
pub use miles_variation::MILES_VARIATION;
pub mod opocensky_variation;
pub use opocensky_variation::OPOCENSKY_VARIATION;
pub mod petrosian_variation;
pub use petrosian_variation::PETROSIAN_VARIATION;
pub mod riumin_variation;
pub use riumin_variation::RIUMIN_VARIATION;
pub mod spassky_system;
pub use spassky_system::SPASSKY_SYSTEM;
pub mod traditional_variation;
pub use traditional_variation::TRADITIONAL_VARIATION;
pub mod yates_variation;
pub use yates_variation::YATES_VARIATION;
