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
# use reco::book::BISHOPS_OPENING;
assert_eq!(BISHOPS_OPENING.original_name(), "Bishop's Opening");
```"#)]
pub static BISHOPS_OPENING: Variation = Variation {
    name: "Bishop's Opening",
    variations: &[&LEWIS_COUNTERGAMBIT,
&LISITSYN_VARIATION,
&BOI_VARIATION,
&LOPEZ_VARIATION,
&MC_DONNELL_GAMBIT,
&FOUR_PAWNS_GAMBIT,
&PHILIDOR_VARIATION,
&PRATT_VARIATION,
&LEWIS_GAMBIT,
&PHILIDOR_COUNTERATTACK,
&THOROLD_GAMBIT,
&DEL_RIO_VARIATION,
&KITCHENER_FOLLY,
&LOPEZ_GAMBIT,
&ANDERSSEN_GAMBIT,
&PONZIANI_GAMBIT,
&URUSOV_GAMBIT,
&VIENNA_HYBRID,
&BERLIN_DEFENSE,
&STEIN_GAMBIT,
&KHAN_GAMBIT,
&CALABRESE_COUNTERGAMBIT,
&HORWITZ_GAMBIT,
&PACHMAN_GAMBIT,
&WARSAW_GAMBIT,
&BODEN_KIESERITZKY_GAMBIT,
&KREJCIK_GAMBIT],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<23>()
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
        from: E7,
        capture: None,
        to: E5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F1,
        capture: None,
        to: C4,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272588421820160),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385432514564),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18441959067824947200),
                white: Bitboard(335605727)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod lewis_countergambit;
pub use lewis_countergambit::LEWIS_COUNTERGAMBIT;
pub mod lisitsyn_variation;
pub use lisitsyn_variation::LISITSYN_VARIATION;
pub mod boi_variation;
pub use boi_variation::BOI_VARIATION;
pub mod lopez_variation;
pub use lopez_variation::LOPEZ_VARIATION;
pub mod mc_donnell_gambit;
pub use mc_donnell_gambit::MC_DONNELL_GAMBIT;
pub mod four_pawns_gambit;
pub use four_pawns_gambit::FOUR_PAWNS_GAMBIT;
pub mod philidor_variation;
pub use philidor_variation::PHILIDOR_VARIATION;
pub mod pratt_variation;
pub use pratt_variation::PRATT_VARIATION;
pub mod lewis_gambit;
pub use lewis_gambit::LEWIS_GAMBIT;
pub mod philidor_counterattack;
pub use philidor_counterattack::PHILIDOR_COUNTERATTACK;
pub mod thorold_gambit;
pub use thorold_gambit::THOROLD_GAMBIT;
pub mod del_rio_variation;
pub use del_rio_variation::DEL_RIO_VARIATION;
pub mod kitchener_folly;
pub use kitchener_folly::KITCHENER_FOLLY;
pub mod lopez_gambit;
pub use lopez_gambit::LOPEZ_GAMBIT;
pub mod anderssen_gambit;
pub use anderssen_gambit::ANDERSSEN_GAMBIT;
pub mod ponziani_gambit;
pub use ponziani_gambit::PONZIANI_GAMBIT;
pub mod urusov_gambit;
pub use urusov_gambit::URUSOV_GAMBIT;
pub mod vienna_hybrid;
pub use vienna_hybrid::VIENNA_HYBRID;
pub mod berlin_defense;
pub use berlin_defense::BERLIN_DEFENSE;
pub mod stein_gambit;
pub use stein_gambit::STEIN_GAMBIT;
pub mod khan_gambit;
pub use khan_gambit::KHAN_GAMBIT;
pub mod calabrese_countergambit;
pub use calabrese_countergambit::CALABRESE_COUNTERGAMBIT;
pub mod horwitz_gambit;
pub use horwitz_gambit::HORWITZ_GAMBIT;
pub mod pachman_gambit;
pub use pachman_gambit::PACHMAN_GAMBIT;
pub mod warsaw_gambit;
pub use warsaw_gambit::WARSAW_GAMBIT;
pub mod boden_kieseritzky_gambit;
pub use boden_kieseritzky_gambit::BODEN_KIESERITZKY_GAMBIT;
pub mod krejcik_gambit;
pub use krejcik_gambit::KREJCIK_GAMBIT;
