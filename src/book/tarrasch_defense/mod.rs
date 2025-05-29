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
# use reco::book::TARRASCH_DEFENSE;
assert_eq!(TARRASCH_DEFENSE.original_name(), "Tarrasch Defense");
```"#)]
pub static TARRASCH_DEFENSE: Variation = Variation {
    name: "Tarrasch Defense",
    variations: &[&TWO_KNIGHTS_VARIATION,
&PRAGUE_VARIATION,
&DUBOV_TARRASCH,
&TARRASCH_GAMBIT,
&RUBINSTEIN_SYSTEM,
&WAGNER_VARIATION,
&CLASSICAL_VARIATION,
&SWEDISH_VARIATION,
&VON_HENNIG_GAMBIT,
&SYMMETRICAL_VARIATION,
&MARSHALL_GAMBIT,
&GRUNFELD_GAMBIT,
&SCHARA_GAMBIT],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::D,
        category: Category::new_static::<32>()
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
        from: D7,
        capture: None,
        to: D5,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C2,
        capture: None,
        to: C4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E7,
        capture: None,
        to: E6,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: B1,
        capture: None,
        to: C3,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C7,
        capture: None,
        to: C5,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63912463640359680),
                knight: Bitboard(4755801206503505984),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18438598943110594560),
                white: Bitboard(201651197)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod two_knights_variation;
pub use two_knights_variation::TWO_KNIGHTS_VARIATION;
pub mod prague_variation;
pub use prague_variation::PRAGUE_VARIATION;
pub mod dubov_tarrasch;
pub use dubov_tarrasch::DUBOV_TARRASCH;
pub mod tarrasch_gambit;
pub use tarrasch_gambit::TARRASCH_GAMBIT;
pub mod rubinstein_system;
pub use rubinstein_system::RUBINSTEIN_SYSTEM;
pub mod wagner_variation;
pub use wagner_variation::WAGNER_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod swedish_variation;
pub use swedish_variation::SWEDISH_VARIATION;
pub mod von_hennig_gambit;
pub use von_hennig_gambit::VON_HENNIG_GAMBIT;
pub mod symmetrical_variation;
pub use symmetrical_variation::SYMMETRICAL_VARIATION;
pub mod marshall_gambit;
pub use marshall_gambit::MARSHALL_GAMBIT;
pub mod grunfeld_gambit;
pub use grunfeld_gambit::GRUNFELD_GAMBIT;
pub mod schara_gambit;
pub use schara_gambit::SCHARA_GAMBIT;
