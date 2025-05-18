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
/// Grünfeld Defense.
pub static GRUNFELD_DEFENSE: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::D,
        category: RangedU8::new_static::<80>(),
    },
    name: Cow::Borrowed(&["Grünfeld Defense"]),
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
        Normal {
            role: Pawn,
            from: D7,
            capture: None,
            to: D5,
            promotion: None,
        },
    ]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(51580324043354880),
                knight: Bitboard(144150372448206912),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(13814615969458290688),
                white: Bitboard(201651197),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    }),
}];
pub mod botvinnik_variation;
pub use botvinnik_variation::BOTVINNIK_VARIATION;
pub mod brinckmann_attack;
pub use brinckmann_attack::BRINCKMANN_ATTACK;
pub mod counterthrust_variation;
pub use counterthrust_variation::COUNTERTHRUST_VARIATION;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod flohr_defense;
pub use flohr_defense::FLOHR_DEFENSE;
pub mod flohr_variation;
pub use flohr_variation::FLOHR_VARIATION;
pub mod gibbon_gambit;
pub use gibbon_gambit::GIBBON_GAMBIT;
pub mod lundin_variation;
pub use lundin_variation::LUNDIN_VARIATION;
pub mod lutikov_variation;
pub use lutikov_variation::LUTIKOV_VARIATION;
pub mod makogonov_variation;
pub use makogonov_variation::MAKOGONOV_VARIATION;
pub mod opocensky_variation;
pub use opocensky_variation::OPOCENSKY_VARIATION;
pub mod pachman_variation;
pub use pachman_variation::PACHMAN_VARIATION;
pub mod russian_variation;
pub use russian_variation::RUSSIAN_VARIATION;
pub mod smyslov_defense;
pub use smyslov_defense::SMYSLOV_DEFENSE;
pub mod stockholm_variation;
pub use stockholm_variation::STOCKHOLM_VARIATION;
pub mod three_knights_variation;
pub use three_knights_variation::THREE_KNIGHTS_VARIATION;
pub mod zaitsev_gambit;
pub use zaitsev_gambit::ZAITSEV_GAMBIT;
