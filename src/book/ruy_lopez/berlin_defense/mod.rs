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

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Ruy Lopez: Berlin Defense.
pub static BERLIN_DEFENSE: [Opening<&str>; 2] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<65>(),
    },
    name: Cow::Borrowed(&["Ruy Lopez", "Berlin Defense"]),
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
        to: B5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272588421820160),
                knight: Bitboard(39582420697090),
                bishop: Bitboard(2594073393955340292),
                rook: Bitboard(9295429630892703777),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606847040)
            },
            ByColor {
                black: Bitboard(13686197443740303360),
                white: Bitboard(8860528495)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703744),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 5,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}, Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<65>(),
    },
    name: Cow::Borrowed(&["Ruy Lopez", "Berlin Defense"]),
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
        to: B5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272588421820160),
                knight: Bitboard(39582420697090),
                bishop: Bitboard(2594073393955340292),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13686197443740303360),
                white: Bitboard(8860528543)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 4,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];pub mod anderssen_variation;
pub use anderssen_variation::ANDERSSEN_VARIATION;
pub mod berlin_wall;
pub use berlin_wall::BERLIN_WALL;
pub mod beverwijk_variation;
pub use beverwijk_variation::BEVERWIJK_VARIATION;
pub mod closed_bernstein_variation;
pub use closed_bernstein_variation::CLOSED_BERNSTEIN_VARIATION;
pub mod closed_showalter_variation;
pub use closed_showalter_variation::CLOSED_SHOWALTER_VARIATION;
pub mod closed_wolf_variation;
pub use closed_wolf_variation::CLOSED_WOLF_VARIATION;
pub mod cordel_variation;
pub use cordel_variation::CORDEL_VARIATION;
pub mod duras_variation;
pub use duras_variation::DURAS_VARIATION;
pub mod fishing_pole_variation;
pub use fishing_pole_variation::FISHING_POLE_VARIATION;
pub mod hedgehog_variation;
pub use hedgehog_variation::HEDGEHOG_VARIATION;
pub mod improved_steinitz_defense;
pub use improved_steinitz_defense::IMPROVED_STEINITZ_DEFENSE;
pub mod kaufmann_variation;
pub use kaufmann_variation::KAUFMANN_VARIATION;
pub mod minckwitz_variation;
pub use minckwitz_variation::MINCKWITZ_VARIATION;
pub mod mortimer_trap;
pub use mortimer_trap::MORTIMER_TRAP;
pub mod mortimer_variation;
pub use mortimer_variation::MORTIMER_VARIATION;
pub mod nyholm_attack;
pub use nyholm_attack::NYHOLM_ATTACK;
pub mod pillsbury_variation;
pub use pillsbury_variation::PILLSBURY_VARIATION;
pub mod rio_gambit_accepted;
pub use rio_gambit_accepted::RIO_GAMBIT_ACCEPTED;
pub mod rio_de_janeiro_variation;
pub use rio_de_janeiro_variation::RIO_DE_JANEIRO_VARIATION;
pub mod rosenthal_variation;
pub use rosenthal_variation::ROSENTHAL_VARIATION;
pub mod tarrasch_trap;
pub use tarrasch_trap::TARRASCH_TRAP;
pub mod trifunovic_variation;
pub use trifunovic_variation::TRIFUNOVIC_VARIATION;
pub mod winawer_attack;
pub use winawer_attack::WINAWER_ATTACK;
pub mod zukertort_variation;
pub use zukertort_variation::ZUKERTORT_VARIATION;
pub mod l_hermet_variation;
pub use l_hermet_variation::L_HERMET_VARIATION;
