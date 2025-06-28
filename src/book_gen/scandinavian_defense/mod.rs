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
# use reco::book::SCANDINAVIAN_DEFENSE;
assert_eq!(SCANDINAVIAN_DEFENSE.original_name(), "Scandinavian Defense");
```"#
)]
pub static SCANDINAVIAN_DEFENSE: Variation = Variation {
    name: "Scandinavian Defense",
    parent: None,
    variations: &[
        &ANDERSSEN_COUNTERATTACK,
        &BLACKBURNE_GAMBIT,
        &BLACKBURNE_KLOOSTERBOER_GAMBIT,
        &BOEHNKE_GAMBIT,
        &BRONSTEIN_VARIATION,
        &CLASSICAL_VARIATION,
        &GRUNFELD_VARIATION,
        &GUBINSKY_MELTS_DEFENSE,
        &ICELANDIC_PALME_GAMBIT,
        &KIEL_VARIATION,
        &KLOOSTERBOER_GAMBIT,
        &KADAS_GAMBIT,
        &LASKER_VARIATION,
        &MAIN_LINE,
        &MARSHALL_VARIATION,
        &MIESES_KOTROC_VARIATION,
        &MODERN_VARIATION,
        &PANOV_TRANSFER,
        &PORTUGUESE_GAMBIT,
        &RICHTER_VARIATION,
        &SCHILLER_PYTEL_VARIATION,
        &VALENCIAN_VARIATION,
        &ZILBERMINTS_GAMBIT,
    ],
    lines: &[
        Line {
            parent: &SCANDINAVIAN_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<0>(),
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
                    from: Square::D7,
                    capture: None,
                    to: Square::D5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
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
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
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
        },
        Line {
            parent: &SCANDINAVIAN_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<0>(),
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
                    from: Square::D7,
                    capture: None,
                    to: Square::D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B2,
                    capture: None,
                    to: Square::B3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
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
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
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
        },
    ],
};
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
pub mod kiel_variation;
pub use kiel_variation::KIEL_VARIATION;
pub mod kloosterboer_gambit;
pub use kloosterboer_gambit::KLOOSTERBOER_GAMBIT;
pub mod kadas_gambit;
pub use kadas_gambit::KADAS_GAMBIT;
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
