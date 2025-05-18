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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Nimzo-Indian Defense: Classical Variation.
pub static CLASSICAL_VARIATION: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::E,
        category: RangedU8::new_static::<32>(),
    },
    name: Cow::Borrowed(&["Nimzo-Indian Defense", "Classical Variation"]),
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
        from: B1,
        capture: None,
        to: C3,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: B4,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D1,
        capture: None,
        to: C2,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67290111821280000),
                knight: Bitboard(144150372448206912),
                bishop: Bitboard(288230376185266212),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303424512),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(11524482748056076288),
                white: Bitboard(201652213)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 3,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];pub mod belyavsky_gambit;
pub use belyavsky_gambit::BELYAVSKY_GAMBIT;
pub mod berlin_variation;
pub use berlin_variation::BERLIN_VARIATION;
pub mod keres_defense;
pub use keres_defense::KERES_DEFENSE;
pub mod milner_barry_variation;
pub use milner_barry_variation::MILNER_BARRY_VARIATION;
pub mod noa_variation;
pub use noa_variation::NOA_VARIATION;
pub mod vitolins_adorjan_gambit;
pub use vitolins_adorjan_gambit::VITOLINS_ADORJAN_GAMBIT;
pub mod zurich_variation;
pub use zurich_variation::ZURICH_VARIATION;
