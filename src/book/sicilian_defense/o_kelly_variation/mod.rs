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
# use reco::book::sicilian_defense::O_KELLY_VARIATION;
assert_eq!(O_KELLY_VARIATION.original_name(), "Sicilian Defense: O'Kelly Variation");
```"#)]
pub static O_KELLY_VARIATION: Variation = Variation {
    name: "O'Kelly Variation",
    variations: &[&RETI_SYSTEM,
&QUIET_SYSTEM,
&NORMAL_SYSTEM,
&KIESERITZKY_SYSTEM,
&YEREVAN_SYSTEM,
&VENICE_SYSTEM,
&MAROCZY_BIND,
&WING_GAMBIT,
&ARONIN_SYSTEM],
    parent: Some(&super::SICILIAN_DEFENSE),
    lines: &[Line {
    code: Code {
        volume: Volume::B,
        category: Category::new_static::<28>()
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
        from: C7,
        capture: None,
        to: C5,
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
        from: A7,
        capture: None,
        to: A6,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(70369861137657600),
                knight: Bitboard(4755801206505340930),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18445056340540784640),
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
};pub mod reti_system;
pub use reti_system::RETI_SYSTEM;
pub mod quiet_system;
pub use quiet_system::QUIET_SYSTEM;
pub mod normal_system;
pub use normal_system::NORMAL_SYSTEM;
pub mod kieseritzky_system;
pub use kieseritzky_system::KIESERITZKY_SYSTEM;
pub mod yerevan_system;
pub use yerevan_system::YEREVAN_SYSTEM;
pub mod venice_system;
pub use venice_system::VENICE_SYSTEM;
pub mod maroczy_bind;
pub use maroczy_bind::MAROCZY_BIND;
pub mod wing_gambit;
pub use wing_gambit::WING_GAMBIT;
pub mod aronin_system;
pub use aronin_system::ARONIN_SYSTEM;
