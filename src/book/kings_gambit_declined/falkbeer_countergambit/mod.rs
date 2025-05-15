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

#[allow(clippy::doc_markdown)]
/// King's Gambit Declined: Falkbeer Countergambit.
pub const FALKBEER_COUNTERGAMBIT: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<31>(),
    },
    name: "King's Gambit Declined",
    variation: &["Falkbeer Countergambit"],
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
            role: Pawn,
            from: F2,
            capture: None,
            to: F4,
            promotion: None,
        },
        Normal {
            role: Pawn,
            from: D7,
            capture: None,
            to: D5,
            promotion: None,
        },
    ],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(65020823504736000),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992),
            },
            ByColor {
                black: Bitboard(18439707302371000320),
                white: Bitboard(805359615),
            },
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
            fullmoves
        } else {
            panic!("fullmoves is zero")
        },
    },
}];
pub mod alapin_variation;
pub use alapin_variation::ALAPIN_VARIATION;
pub mod anderssen_attack;
pub use anderssen_attack::ANDERSSEN_ATTACK;
pub mod blackburne_attack;
pub use blackburne_attack::BLACKBURNE_ATTACK;
pub mod charousek_gambit;
pub use charousek_gambit::CHAROUSEK_GAMBIT;
pub mod charousek_gambit_accepted;
pub use charousek_gambit_accepted::CHAROUSEK_GAMBIT_ACCEPTED;
pub mod charousek_variation;
pub use charousek_variation::CHAROUSEK_VARIATION;
pub mod hinrichsen_gambit;
pub use hinrichsen_gambit::HINRICHSEN_GAMBIT;
pub mod miles_gambit;
pub use miles_gambit::MILES_GAMBIT;
pub mod milner_barry_variation;
pub use milner_barry_variation::MILNER_BARRY_VARIATION;
pub mod modern_transfer;
pub use modern_transfer::MODERN_TRANSFER;
pub mod nimzowitsch_marshall_countergambit;
pub use nimzowitsch_marshall_countergambit::NIMZOWITSCH_MARSHALL_COUNTERGAMBIT;
pub mod pickler_gambit;
pub use pickler_gambit::PICKLER_GAMBIT;
pub mod rubinstein_variation;
pub use rubinstein_variation::RUBINSTEIN_VARIATION;
pub mod staunton_line;
pub use staunton_line::STAUNTON_LINE;
pub mod tarrasch_variation;
pub use tarrasch_variation::TARRASCH_VARIATION;
