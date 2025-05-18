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
/// Polish Opening.
pub static POLISH_OPENING: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::A,
        category: RangedU8::new_static::<0>(),
    },
    name: Cow::Borrowed(&["Polish Opening"]),
    moves: Cow::Borrowed(&[Normal {
        role: Pawn,
        from: B2,
        capture: None,
        to: B4,
        promotion: None,
    }]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(71776119094836480),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(18446462598732840960),
                white: Bitboard(33619455),
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
    }),
}];
pub mod baltic_defense;
pub use baltic_defense::BALTIC_DEFENSE;
pub mod birmingham_gambit;
pub use birmingham_gambit::BIRMINGHAM_GAMBIT;
pub mod bugayev_advance_variation;
pub use bugayev_advance_variation::BUGAYEV_ADVANCE_VARIATION;
pub mod bugayev_attack;
pub use bugayev_attack::BUGAYEV_ATTACK;
pub mod czech_defense;
pub use czech_defense::CZECH_DEFENSE;
pub mod dutch_defense;
pub use dutch_defense::DUTCH_DEFENSE;
pub mod german_defense;
pub use german_defense::GERMAN_DEFENSE;
pub mod grigorian_variation;
pub use grigorian_variation::GRIGORIAN_VARIATION;
pub mod karniewski_variation;
pub use karniewski_variation::KARNIEWSKI_VARIATION;
pub mod kings_indian_variation;
pub use kings_indian_variation::KINGS_INDIAN_VARIATION;
pub mod myers_variation;
pub use myers_variation::MYERS_VARIATION;
pub mod outflank_variation;
pub use outflank_variation::OUTFLANK_VARIATION;
pub mod queens_indian_variation;
pub use queens_indian_variation::QUEENS_INDIAN_VARIATION;
pub mod queenside_defense;
pub use queenside_defense::QUEENSIDE_DEFENSE;
pub mod rooks_swap_line;
pub use rooks_swap_line::ROOKS_SWAP_LINE;
pub mod schiffler_sokolsky_variation;
pub use schiffler_sokolsky_variation::SCHIFFLER_SOKOLSKY_VARIATION;
pub mod schuehler_gambit;
pub use schuehler_gambit::SCHUEHLER_GAMBIT;
pub mod symmetrical_variation;
pub use symmetrical_variation::SYMMETRICAL_VARIATION;
pub mod tartakower_gambit;
pub use tartakower_gambit::TARTAKOWER_GAMBIT;
pub mod wolferts_gambit;
pub use wolferts_gambit::WOLFERTS_GAMBIT;
pub mod zukertort_system;
pub use zukertort_system::ZUKERTORT_SYSTEM;
