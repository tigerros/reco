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
# use reco::book::BIRD_OPENING;
assert_eq!(BIRD_OPENING.original_name(), "Bird Opening");
```"#)]
pub static BIRD_OPENING: Variation = Variation {
    name: "Bird Opening",
    variations: &[&HOBBS_GAMBIT,
&LASKER_GAMBIT,
&WAGNER_ZWITERSCH_GAMBIT,
&SCHLECHTER_GAMBIT,
&DUTCH_VARIATION,
&HORSEFLY_DEFENSE,
&MUJANNAH,
&STURM_GAMBIT,
&SWISS_GAMBIT,
&MYERS_DEFENSE,
&LASKER_VARIATION,
&BATAVO_POLISH_ATTACK,
&HOBBS_ZILBERMINTS_GAMBIT,
&THOMAS_GAMBIT,
&WILLIAMS_GAMBIT,
&WILLIAMS_ZILBERMINTS_GAMBIT,
&FROMS_GAMBIT,
&SIEGENER_GAMBIT,
&PLATZ_GAMBIT,
&DOUBLE_DUCK_FORMATION],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::A,
        category: Category::new_static::<2>()
    },
    moves: &[
    Normal {
        role: Pawn,
        from: F2,
        capture: None,
        to: F4,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(71776119598145280),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18446462598732840960),
                white: Bitboard(536928255)
            }
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
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod hobbs_gambit;
pub use hobbs_gambit::HOBBS_GAMBIT;
pub mod lasker_gambit;
pub use lasker_gambit::LASKER_GAMBIT;
pub mod wagner_zwitersch_gambit;
pub use wagner_zwitersch_gambit::WAGNER_ZWITERSCH_GAMBIT;
pub mod schlechter_gambit;
pub use schlechter_gambit::SCHLECHTER_GAMBIT;
pub mod dutch_variation;
pub use dutch_variation::DUTCH_VARIATION;
pub mod horsefly_defense;
pub use horsefly_defense::HORSEFLY_DEFENSE;
pub mod mujannah;
pub use mujannah::MUJANNAH;
pub mod sturm_gambit;
pub use sturm_gambit::STURM_GAMBIT;
pub mod swiss_gambit;
pub use swiss_gambit::SWISS_GAMBIT;
pub mod myers_defense;
pub use myers_defense::MYERS_DEFENSE;
pub mod lasker_variation;
pub use lasker_variation::LASKER_VARIATION;
pub mod batavo_polish_attack;
pub use batavo_polish_attack::BATAVO_POLISH_ATTACK;
pub mod hobbs_zilbermints_gambit;
pub use hobbs_zilbermints_gambit::HOBBS_ZILBERMINTS_GAMBIT;
pub mod thomas_gambit;
pub use thomas_gambit::THOMAS_GAMBIT;
pub mod williams_gambit;
pub use williams_gambit::WILLIAMS_GAMBIT;
pub mod williams_zilbermints_gambit;
pub use williams_zilbermints_gambit::WILLIAMS_ZILBERMINTS_GAMBIT;
pub mod froms_gambit;
pub use froms_gambit::FROMS_GAMBIT;
pub mod siegener_gambit;
pub use siegener_gambit::SIEGENER_GAMBIT;
pub mod platz_gambit;
pub use platz_gambit::PLATZ_GAMBIT;
pub mod double_duck_formation;
pub use double_duck_formation::DOUBLE_DUCK_FORMATION;
