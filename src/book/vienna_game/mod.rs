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
# use reco::book::VIENNA_GAME;
assert_eq!(VIENNA_GAME.original_name(), "Vienna Game");
```"#)]
pub static VIENNA_GAME: Variation = Variation {
    name: "Vienna Game",
    variations: &[&HAMPPE_MUZIO,
&OMAHA_GAMBIT,
&MAX_LANGE_DEFENSE,
&PAULSEN_VARIATION,
&FRANKENSTEIN_DRACULA_VARIATION,
&PHILIDOR_COUNTERGAMBIT,
&ADAMS_GAMBIT,
&HAMPPE_MEITNER_VARIATION,
&HAMPPE_MUZIO_GAMBIT,
&MIESES_VARIATION,
&FALKBEER_VARIATION,
&GIRAFFE_ATTACK,
&ANDERSSEN_DEFENSE,
&PIERCE_GAMBIT,
&ZHURAVLEV_COUNTERGAMBIT,
&HEYDE_VARIATION,
&FYFE_GAMBIT,
&VIENNA_GAMBIT,
&STANLEY_VARIATION,
&MENGARINI_VARIATION],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<25>()
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
        role: Knight,
        from: B1,
        capture: None,
        to: C3,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272588421820160),
                knight: Bitboard(4755801206503505984),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18441959067824947200),
                white: Bitboard(268759037)
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
};pub mod hamppe_muzio;
pub use hamppe_muzio::HAMPPE_MUZIO;
pub mod omaha_gambit;
pub use omaha_gambit::OMAHA_GAMBIT;
pub mod max_lange_defense;
pub use max_lange_defense::MAX_LANGE_DEFENSE;
pub mod paulsen_variation;
pub use paulsen_variation::PAULSEN_VARIATION;
pub mod frankenstein_dracula_variation;
pub use frankenstein_dracula_variation::FRANKENSTEIN_DRACULA_VARIATION;
pub mod philidor_countergambit;
pub use philidor_countergambit::PHILIDOR_COUNTERGAMBIT;
pub mod adams_gambit;
pub use adams_gambit::ADAMS_GAMBIT;
pub mod hamppe_meitner_variation;
pub use hamppe_meitner_variation::HAMPPE_MEITNER_VARIATION;
pub mod hamppe_muzio_gambit;
pub use hamppe_muzio_gambit::HAMPPE_MUZIO_GAMBIT;
pub mod mieses_variation;
pub use mieses_variation::MIESES_VARIATION;
pub mod falkbeer_variation;
pub use falkbeer_variation::FALKBEER_VARIATION;
pub mod giraffe_attack;
pub use giraffe_attack::GIRAFFE_ATTACK;
pub mod anderssen_defense;
pub use anderssen_defense::ANDERSSEN_DEFENSE;
pub mod pierce_gambit;
pub use pierce_gambit::PIERCE_GAMBIT;
pub mod zhuravlev_countergambit;
pub use zhuravlev_countergambit::ZHURAVLEV_COUNTERGAMBIT;
pub mod heyde_variation;
pub use heyde_variation::HEYDE_VARIATION;
pub mod fyfe_gambit;
pub use fyfe_gambit::FYFE_GAMBIT;
pub mod vienna_gambit;
pub use vienna_gambit::VIENNA_GAMBIT;
pub mod stanley_variation;
pub use stanley_variation::STANLEY_VARIATION;
pub mod mengarini_variation;
pub use mengarini_variation::MENGARINI_VARIATION;
