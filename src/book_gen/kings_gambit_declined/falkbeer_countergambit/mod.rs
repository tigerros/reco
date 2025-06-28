#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use crate::{Category, Code, Line, Variation, Volume};
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use core::num::NonZeroU32;
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
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::bitboard::Bitboard;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::board::Board;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::{ByColor, ByRole, Setup};
#[cfg_attr(
    feature = "alloc",
    doc = r#"```rust
# use reco::book::kings_gambit_declined::FALKBEER_COUNTERGAMBIT;
assert_eq!(FALKBEER_COUNTERGAMBIT.original_name(), "King's Gambit Declined: Falkbeer Countergambit");
```"#
)]
pub static FALKBEER_COUNTERGAMBIT: Variation = Variation {
    name: "Falkbeer Countergambit",
    parent: Some(&super::KINGS_GAMBIT_DECLINED),
    variations: &[
        &ALAPIN_VARIATION,
        &ANDERSSEN_ATTACK,
        &BLACKBURNE_ATTACK,
        &CHAROUSEK_GAMBIT,
        &CHAROUSEK_GAMBIT_ACCEPTED,
        &CHAROUSEK_VARIATION,
        &HINRICHSEN_GAMBIT,
        &MILES_GAMBIT,
        &MILNER_BARRY_VARIATION,
        &MODERN_TRANSFER,
        &NIMZOWITSCH_MARSHALL_COUNTERGAMBIT,
        &PICKLER_GAMBIT,
        &RUBINSTEIN_VARIATION,
        &STAUNTON_LINE,
        &TARRASCH_VARIATION,
    ],
    lines: &[Line {
        parent: &FALKBEER_COUNTERGAMBIT,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<3>(),
        },
        moves: &[
            Normal {
                role: Pawn,
                from: Square::E2,
                capture: None,
                to: Square::E4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E7,
                capture: None,
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::F2,
                capture: None,
                to: Square::F4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::D7,
                capture: None,
                to: Square::D5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
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
            ) {
                board
            } else {
                #[expect(
                    clippy::unreachable,
                    reason = "intentional. It's in a const expression"
                )]
                {
                    unreachable!()
                }
            },
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
                #[expect(
                    clippy::unreachable,
                    reason = "intentional. It's in a const expression"
                )]
                {
                    unreachable!()
                }
            },
        },
    }],
};
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
