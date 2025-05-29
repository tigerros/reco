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
# use reco::book::NIMZOWITSCH_DEFENSE;
assert_eq!(NIMZOWITSCH_DEFENSE.original_name(), "Nimzowitsch Defense");
```"#)]
pub static NIMZOWITSCH_DEFENSE: Variation = Variation {
    name: "Nimzowitsch Defense",
    variations: &[&WHEELER_GAMBIT,
&HORNUNG_GAMBIT,
&COLORADO_COUNTERGAMBIT,
&WOODCHUCK_VARIATION,
&NEO_MONGOLOID_DEFENSE,
&FRENCH_CONNECTION,
&DECLINED_VARIATION,
&KENNEDY_VARIATION,
&EL_COLUMPIO_DEFENSE,
&BREYER_VARIATION,
&PSEUDO_SPANISH_VARIATION,
&COLORADO_COUNTERGAMBIT_ACCEPTED,
&PIRC_CONNECTION,
&FRANCO_NIMZOWITSCH_VARIATION,
&MIKENAS_VARIATION,
&SCANDINAVIAN_VARIATION,
&WILLIAMS_VARIATION],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::B,
        category: Category::new_static::<0>()
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
        role: Knight,
        from: B8,
        capture: None,
        to: C6,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(71776119329713920),
                knight: Bitboard(4611690416473899074),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18302351808703496192),
                white: Bitboard(268496895)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
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
},
Line {
    code: Code {
        volume: Volume::B,
        category: Category::new_static::<0>()
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
        role: Knight,
        from: B8,
        capture: None,
        to: C6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D4,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(71776119463929600),
                knight: Bitboard(4611690416473899074),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18302351808703496192),
                white: Bitboard(402712575)
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
};pub mod wheeler_gambit;
pub use wheeler_gambit::WHEELER_GAMBIT;
pub mod hornung_gambit;
pub use hornung_gambit::HORNUNG_GAMBIT;
pub mod colorado_countergambit;
pub use colorado_countergambit::COLORADO_COUNTERGAMBIT;
pub mod woodchuck_variation;
pub use woodchuck_variation::WOODCHUCK_VARIATION;
pub mod neo_mongoloid_defense;
pub use neo_mongoloid_defense::NEO_MONGOLOID_DEFENSE;
pub mod french_connection;
pub use french_connection::FRENCH_CONNECTION;
pub mod declined_variation;
pub use declined_variation::DECLINED_VARIATION;
pub mod kennedy_variation;
pub use kennedy_variation::KENNEDY_VARIATION;
pub mod el_columpio_defense;
pub use el_columpio_defense::EL_COLUMPIO_DEFENSE;
pub mod breyer_variation;
pub use breyer_variation::BREYER_VARIATION;
pub mod pseudo_spanish_variation;
pub use pseudo_spanish_variation::PSEUDO_SPANISH_VARIATION;
pub mod colorado_countergambit_accepted;
pub use colorado_countergambit_accepted::COLORADO_COUNTERGAMBIT_ACCEPTED;
pub mod pirc_connection;
pub use pirc_connection::PIRC_CONNECTION;
pub mod franco_nimzowitsch_variation;
pub use franco_nimzowitsch_variation::FRANCO_NIMZOWITSCH_VARIATION;
pub mod mikenas_variation;
pub use mikenas_variation::MIKENAS_VARIATION;
pub mod scandinavian_variation;
pub use scandinavian_variation::SCANDINAVIAN_VARIATION;
pub mod williams_variation;
pub use williams_variation::WILLIAMS_VARIATION;
