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
# use reco::book::ruy_lopez::SCHLIEMANN_DEFENSE;
assert_eq!(SCHLIEMANN_DEFENSE.original_name(), "Ruy Lopez: Schliemann Defense");
```"#)]
pub static SCHLIEMANN_DEFENSE: Variation = Variation {
    name: "Schliemann Defense",
    variations: &[&DYCKHOFF_VARIATION,
&TARTAKOWER_VARIATION,
&CLASSICAL_VARIATION,
&KOSTIC_DEFENSE,
&EXCHANGE_VARIATION,
&SCHONEMANN_ATTACK,
&JAENISCH_GAMBIT_ACCEPTED,
&MOHRING_VARIATION],
    parent: Some(&super::RUY_LOPEZ),
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<63>()
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
        role: Knight,
        from: B8,
        capture: None,
        to: C6,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F1,
        capture: None,
        to: B5,
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
                knight: Bitboard(4611690416475996162),
                bishop: Bitboard(2594073393955340292),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18288841215979814912),
                white: Bitboard(8860528543)
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
};pub mod dyckhoff_variation;
pub use dyckhoff_variation::DYCKHOFF_VARIATION;
pub mod tartakower_variation;
pub use tartakower_variation::TARTAKOWER_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod kostic_defense;
pub use kostic_defense::KOSTIC_DEFENSE;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod schonemann_attack;
pub use schonemann_attack::SCHONEMANN_ATTACK;
pub mod jaenisch_gambit_accepted;
pub use jaenisch_gambit_accepted::JAENISCH_GAMBIT_ACCEPTED;
pub mod mohring_variation;
pub use mohring_variation::MOHRING_VARIATION;
