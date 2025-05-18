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
/// Bogo-Indian Defense.
pub static BOGO_INDIAN_DEFENSE: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::E,
        category: RangedU8::new_static::<11>(),
    },
    name: Cow::Borrowed(&["Bogo-Indian Defense"]),
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
            role: Bishop,
            from: F8,
            capture: None,
            to: B4,
            promotion: None,
        },
    ]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67290111821280000),
                knight: Bitboard(144150372450041858),
                bishop: Bitboard(288230376185266212),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(11524482748056076288),
                white: Bitboard(203486143),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 2,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    }),
}];
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod grunfeld_variation;
pub use grunfeld_variation::GRUNFELD_VARIATION;
pub mod haiti_variation;
pub use haiti_variation::HAITI_VARIATION;
pub mod monticelli_trap;
pub use monticelli_trap::MONTICELLI_TRAP;
pub mod new_england_variation;
pub use new_england_variation::NEW_ENGLAND_VARIATION;
pub mod nimzowitsch_variation;
pub use nimzowitsch_variation::NIMZOWITSCH_VARIATION;
pub mod retreat_variation;
pub use retreat_variation::RETREAT_VARIATION;
pub mod vitolins_variation;
pub use vitolins_variation::VITOLINS_VARIATION;
pub mod wade_smyslov_variation;
pub use wade_smyslov_variation::WADE_SMYSLOV_VARIATION;
