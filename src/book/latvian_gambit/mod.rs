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
# use reco::book::LATVIAN_GAMBIT;
assert_eq!(LATVIAN_GAMBIT.original_name(), "Latvian Gambit");
```"#)]
pub static LATVIAN_GAMBIT: Variation = Variation {
    name: "Latvian Gambit",
    variations: &[&MASON_COUNTERGAMBIT,
&MAYET_ATTACK,
&FRASER_DEFENSE,
&DIEPSTRATEN_COUNTERGAMBIT,
&BEHTING_VARIATION,
&LOBSTER_GAMBIT,
&CORKSCREW_COUNTERGAMBIT,
&CORKSCREW_GAMBIT,
&GRECO_VARIATION,
&MLOTKOWSKI_VARIATION,
&SENECHAUD_GAMBIT,
&CLAM_GAMBIT],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<40>()
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
        from: G1,
        capture: None,
        to: F3,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: F7,
        capture: None,
        to: F5,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(58265526606032640),
                knight: Bitboard(4755801206505340930),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18432952006009159680),
                white: Bitboard(270593983)
            }
        ),
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
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod mason_countergambit;
pub use mason_countergambit::MASON_COUNTERGAMBIT;
pub mod mayet_attack;
pub use mayet_attack::MAYET_ATTACK;
pub mod fraser_defense;
pub use fraser_defense::FRASER_DEFENSE;
pub mod diepstraten_countergambit;
pub use diepstraten_countergambit::DIEPSTRATEN_COUNTERGAMBIT;
pub mod behting_variation;
pub use behting_variation::BEHTING_VARIATION;
pub mod lobster_gambit;
pub use lobster_gambit::LOBSTER_GAMBIT;
pub mod corkscrew_countergambit;
pub use corkscrew_countergambit::CORKSCREW_COUNTERGAMBIT;
pub mod corkscrew_gambit;
pub use corkscrew_gambit::CORKSCREW_GAMBIT;
pub mod greco_variation;
pub use greco_variation::GRECO_VARIATION;
pub mod mlotkowski_variation;
pub use mlotkowski_variation::MLOTKOWSKI_VARIATION;
pub mod senechaud_gambit;
pub use senechaud_gambit::SENECHAUD_GAMBIT;
pub mod clam_gambit;
pub use clam_gambit::CLAM_GAMBIT;
