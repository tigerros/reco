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
/// Scandinavian Defense.
pub static SCANDINAVIAN_DEFENSE: [Opening<&str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<1>(),
        },
        name: Cow::Borrowed(&["Scandinavian Defense"]),
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
                from: D7,
                capture: None,
                to: D5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: B2,
                capture: None,
                to: B3,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(69524353875897600),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18444210833278894080),
                    white: Bitboard(268627455),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::B,
            category: RangedU8::new_static::<1>(),
        },
        name: Cow::Borrowed(&["Scandinavian Defense"]),
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
                from: D7,
                capture: None,
                to: D5,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(69524353875767040),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18444210833278894080),
                    white: Bitboard(268496895),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
];
pub mod anderssen_counterattack;
pub use anderssen_counterattack::ANDERSSEN_COUNTERATTACK;
pub mod blackburne_gambit;
pub use blackburne_gambit::BLACKBURNE_GAMBIT;
pub mod blackburne_kloosterboer_gambit;
pub use blackburne_kloosterboer_gambit::BLACKBURNE_KLOOSTERBOER_GAMBIT;
pub mod boehnke_gambit;
pub use boehnke_gambit::BOEHNKE_GAMBIT;
pub mod bronstein_variation;
pub use bronstein_variation::BRONSTEIN_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod grunfeld_variation;
pub use grunfeld_variation::GRUNFELD_VARIATION;
pub mod gubinsky_melts_defense;
pub use gubinsky_melts_defense::GUBINSKY_MELTS_DEFENSE;
pub mod icelandic_palme_gambit;
pub use icelandic_palme_gambit::ICELANDIC_PALME_GAMBIT;
pub mod kadas_gambit;
pub use kadas_gambit::KADAS_GAMBIT;
pub mod kiel_variation;
pub use kiel_variation::KIEL_VARIATION;
pub mod kloosterboer_gambit;
pub use kloosterboer_gambit::KLOOSTERBOER_GAMBIT;
pub mod lasker_variation;
pub use lasker_variation::LASKER_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod marshall_variation;
pub use marshall_variation::MARSHALL_VARIATION;
pub mod mieses_kotroc_variation;
pub use mieses_kotroc_variation::MIESES_KOTROC_VARIATION;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod panov_transfer;
pub use panov_transfer::PANOV_TRANSFER;
pub mod portuguese_gambit;
pub use portuguese_gambit::PORTUGUESE_GAMBIT;
pub mod richter_variation;
pub use richter_variation::RICHTER_VARIATION;
pub mod schiller_pytel_variation;
pub use schiller_pytel_variation::SCHILLER_PYTEL_VARIATION;
pub mod valencian_variation;
pub use valencian_variation::VALENCIAN_VARIATION;
pub mod zilbermints_gambit;
pub use zilbermints_gambit::ZILBERMINTS_GAMBIT;
