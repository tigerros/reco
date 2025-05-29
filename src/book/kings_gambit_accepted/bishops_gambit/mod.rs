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
# use reco::book::kings_gambit_accepted::BISHOPS_GAMBIT;
assert_eq!(BISHOPS_GAMBIT.original_name(), "King's Gambit Accepted: Bishop's Gambit");
```"#)]
pub static BISHOPS_GAMBIT: Variation = Variation {
    name: "Bishop's Gambit",
    variations: &[&BOGOLJUBOW_DEFENSE,
&ANDERSSEN_DEFENSE,
&MAURIAN_DEFENSE,
&BODEN_VARIATION,
&COZIO_DEFENSE,
&ANDERSSEN_VARIATION,
&BLEDOW_COUNTERGAMBIT,
&BOGOLJUBOW_VARIATION,
&BOREN_SVENONIUS_VARIATION,
&GRECO_VARIATION,
&GIANUTIO_GAMBIT,
&GRIMM_ATTACK,
&LOPEZ_VARIATION,
&MC_DONNELL_ATTACK,
&BLEDOW_VARIATION,
&CHIGORINS_ATTACK,
&CLASSICAL_DEFENSE,
&FRASER_VARIATION,
&KIESERITZKY_GAMBIT,
&PAULSEN_ATTACK,
&COZIO_VARIATION,
&STEINITZ_DEFENSE,
&FIRST_JAENISCH_VARIATION,
&BRYAN_COUNTERGAMBIT,
&LOPEZ_DEFENSE],
    parent: Some(&super::KINGS_GAMBIT_ACCEPTED),
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<33>()
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
        role: Pawn,
        from: F2,
        capture: None,
        to: F4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E5,
        capture: Some(
            Pawn,
        ),
        to: F4,
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
                pawn: Bitboard(67272520239206144),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385432514564),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18441958999642341376),
                white: Bitboard(335597535)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod bogoljubow_defense;
pub use bogoljubow_defense::BOGOLJUBOW_DEFENSE;
pub mod anderssen_defense;
pub use anderssen_defense::ANDERSSEN_DEFENSE;
pub mod maurian_defense;
pub use maurian_defense::MAURIAN_DEFENSE;
pub mod boden_variation;
pub use boden_variation::BODEN_VARIATION;
pub mod cozio_defense;
pub use cozio_defense::COZIO_DEFENSE;
pub mod anderssen_variation;
pub use anderssen_variation::ANDERSSEN_VARIATION;
pub mod bledow_countergambit;
pub use bledow_countergambit::BLEDOW_COUNTERGAMBIT;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod boren_svenonius_variation;
pub use boren_svenonius_variation::BOREN_SVENONIUS_VARIATION;
pub mod greco_variation;
pub use greco_variation::GRECO_VARIATION;
pub mod gianutio_gambit;
pub use gianutio_gambit::GIANUTIO_GAMBIT;
pub mod grimm_attack;
pub use grimm_attack::GRIMM_ATTACK;
pub mod lopez_variation;
pub use lopez_variation::LOPEZ_VARIATION;
pub mod mc_donnell_attack;
pub use mc_donnell_attack::MC_DONNELL_ATTACK;
pub mod bledow_variation;
pub use bledow_variation::BLEDOW_VARIATION;
pub mod chigorins_attack;
pub use chigorins_attack::CHIGORINS_ATTACK;
pub mod classical_defense;
pub use classical_defense::CLASSICAL_DEFENSE;
pub mod fraser_variation;
pub use fraser_variation::FRASER_VARIATION;
pub mod kieseritzky_gambit;
pub use kieseritzky_gambit::KIESERITZKY_GAMBIT;
pub mod paulsen_attack;
pub use paulsen_attack::PAULSEN_ATTACK;
pub mod cozio_variation;
pub use cozio_variation::COZIO_VARIATION;
pub mod steinitz_defense;
pub use steinitz_defense::STEINITZ_DEFENSE;
pub mod first_jaenisch_variation;
pub use first_jaenisch_variation::FIRST_JAENISCH_VARIATION;
pub mod bryan_countergambit;
pub use bryan_countergambit::BRYAN_COUNTERGAMBIT;
pub mod lopez_defense;
pub use lopez_defense::LOPEZ_DEFENSE;
