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
# use reco::book::ENGLUND_GAMBIT;
assert_eq!(ENGLUND_GAMBIT.original_name(), "Englund Gambit");
```"#)]
pub static ENGLUND_GAMBIT: Variation = Variation {
    name: "Englund Gambit",
    variations: &[&MAIN_LINE,
&ZILBERMINTS_GAMBIT,
&STOCKHOLM_VARIATION,
&SOLLER_GAMBIT_DEFERRED,
&SOLLER_GAMBIT,
&MOSQUITO_GAMBIT,
&FELBECKER_GAMBIT,
&HARTLAUB_CHARLICK_GAMBIT],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::A,
        category: Category::new_static::<40>()
    },
    moves: &[
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E7,
        capture: None,
        to: E5,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272588287604480),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18441959067824947200),
                white: Bitboard(134281215)
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
}]
};pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod zilbermints_gambit;
pub use zilbermints_gambit::ZILBERMINTS_GAMBIT;
pub mod stockholm_variation;
pub use stockholm_variation::STOCKHOLM_VARIATION;
pub mod soller_gambit_deferred;
pub use soller_gambit_deferred::SOLLER_GAMBIT_DEFERRED;
pub mod soller_gambit;
pub use soller_gambit::SOLLER_GAMBIT;
pub mod mosquito_gambit;
pub use mosquito_gambit::MOSQUITO_GAMBIT;
pub mod felbecker_gambit;
pub use felbecker_gambit::FELBECKER_GAMBIT;
pub mod hartlaub_charlick_gambit;
pub use hartlaub_charlick_gambit::HARTLAUB_CHARLICK_GAMBIT;
