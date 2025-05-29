#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::bitboard::Bitboard;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::board::Board;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::{ByRole, ByColor, Setup};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use core::num::NonZeroU32;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use crate::{Variation, Line, Code, Volume, Category};#[cfg_attr(feature = "alloc", doc = r#"```rust
# use reco::book::SCANDINAVIAN_DEFENSE;
assert_eq!(SCANDINAVIAN_DEFENSE.original_name(), "Scandinavian Defense");
```"#)]
pub static SCANDINAVIAN_DEFENSE: Variation = Variation {
    name: "Scandinavian Defense",
    variations: &[&CLASSICAL_VARIATION,
&SCHILLER_PYTEL_VARIATION,
&GUBINSKY_MELTS_DEFENSE,
&PORTUGUESE_GAMBIT,
&ICELANDIC_PALME_GAMBIT,
&VALENCIAN_VARIATION,
&BRONSTEIN_VARIATION,
&BLACKBURNE_GAMBIT,
&MAIN_LINE,
&KLOOSTERBOER_GAMBIT,
&MARSHALL_VARIATION,
&PANOV_TRANSFER,
&KADAS_GAMBIT,
&GRUNFELD_VARIATION,
&BOEHNKE_GAMBIT,
&ANDERSSEN_COUNTERATTACK,
&MIESES_KOTROC_VARIATION,
&ZILBERMINTS_GAMBIT,
&BLACKBURNE_KLOOSTERBOER_GAMBIT,
&KIEL_VARIATION,
&LASKER_VARIATION,
&MODERN_VARIATION,
&RICHTER_VARIATION],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::B,
        category: Category::new_static::<1>()
    },
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
        from: D7,
        capture: None,
        to: D5,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(69524353875767040),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18444210833278894080),
                white: Bitboard(268496895)
            }
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
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
},
Line {
    code: Code {
        volume: Volume::B,
        category: Category::new_static::<1>()
    },
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
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(69524353875897600),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18444210833278894080),
                white: Bitboard(268627455)
            }
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
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod schiller_pytel_variation;
pub use schiller_pytel_variation::SCHILLER_PYTEL_VARIATION;
pub mod gubinsky_melts_defense;
pub use gubinsky_melts_defense::GUBINSKY_MELTS_DEFENSE;
pub mod portuguese_gambit;
pub use portuguese_gambit::PORTUGUESE_GAMBIT;
pub mod icelandic_palme_gambit;
pub use icelandic_palme_gambit::ICELANDIC_PALME_GAMBIT;
pub mod valencian_variation;
pub use valencian_variation::VALENCIAN_VARIATION;
pub mod bronstein_variation;
pub use bronstein_variation::BRONSTEIN_VARIATION;
pub mod blackburne_gambit;
pub use blackburne_gambit::BLACKBURNE_GAMBIT;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod kloosterboer_gambit;
pub use kloosterboer_gambit::KLOOSTERBOER_GAMBIT;
pub mod marshall_variation;
pub use marshall_variation::MARSHALL_VARIATION;
pub mod panov_transfer;
pub use panov_transfer::PANOV_TRANSFER;
pub mod kadas_gambit;
pub use kadas_gambit::KADAS_GAMBIT;
pub mod grunfeld_variation;
pub use grunfeld_variation::GRUNFELD_VARIATION;
pub mod boehnke_gambit;
pub use boehnke_gambit::BOEHNKE_GAMBIT;
pub mod anderssen_counterattack;
pub use anderssen_counterattack::ANDERSSEN_COUNTERATTACK;
pub mod mieses_kotroc_variation;
pub use mieses_kotroc_variation::MIESES_KOTROC_VARIATION;
pub mod zilbermints_gambit;
pub use zilbermints_gambit::ZILBERMINTS_GAMBIT;
pub mod blackburne_kloosterboer_gambit;
pub use blackburne_kloosterboer_gambit::BLACKBURNE_KLOOSTERBOER_GAMBIT;
pub mod kiel_variation;
pub use kiel_variation::KIEL_VARIATION;
pub mod lasker_variation;
pub use lasker_variation::LASKER_VARIATION;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod richter_variation;
pub use richter_variation::RICHTER_VARIATION;
