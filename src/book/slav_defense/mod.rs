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
# use reco::book::SLAV_DEFENSE;
assert_eq!(SLAV_DEFENSE.original_name(), "Slav Defense");
```"#)]
pub static SLAV_DEFENSE: Variation = Variation {
    name: "Slav Defense",
    variations: &[&WINAWER_COUNTERGAMBIT,
&BREYER_VARIATION,
&ALAPIN_VARIATION,
&BONET_GAMBIT,
&DIEMER_GAMBIT,
&CHEBANENKO_VARIATION,
&GELLER_GAMBIT,
&SMYSLOV_VARIATION,
&SOULTANBEIEFF_VARIATION,
&CZECH_VARIATION,
&STEINER_VARIATION,
&SLAV_GAMBIT,
&SCHLECHTER_VARIATION,
&MODERN_LINE,
&SUCHTING_VARIATION,
&TWO_KNIGHTS_ATTACK,
&ALEKHINE_VARIATION,
&QUIET_VARIATION,
&THREE_KNIGHTS_VARIATION,
&EXCHANGE_VARIATION],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::D,
        category: Category::new_static::<10>()
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
        from: C7,
        capture: None,
        to: C6,
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
        from: D5,
        capture: Some(
            Pawn,
        ),
        to: C4,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(68402817588589312),
                knight: Bitboard(4755801206503505984),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18443089297125933056),
                white: Bitboard(134542333)
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
},
Line {
    code: Code {
        volume: Volume::D,
        category: Category::new_static::<10>()
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
        from: C7,
        capture: None,
        to: C6,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(68402851948327680),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18443089331418562560),
                white: Bitboard(201389055)
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
},
Line {
    code: Code {
        volume: Volume::D,
        category: Category::new_static::<10>()
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
        from: C7,
        capture: None,
        to: C6,
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
                pawn: Bitboard(68402851948327680),
                knight: Bitboard(4755801206503505984),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18443089331418562560),
                white: Bitboard(201651197)
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
};pub mod winawer_countergambit;
pub use winawer_countergambit::WINAWER_COUNTERGAMBIT;
pub mod breyer_variation;
pub use breyer_variation::BREYER_VARIATION;
pub mod alapin_variation;
pub use alapin_variation::ALAPIN_VARIATION;
pub mod bonet_gambit;
pub use bonet_gambit::BONET_GAMBIT;
pub mod diemer_gambit;
pub use diemer_gambit::DIEMER_GAMBIT;
pub mod chebanenko_variation;
pub use chebanenko_variation::CHEBANENKO_VARIATION;
pub mod geller_gambit;
pub use geller_gambit::GELLER_GAMBIT;
pub mod smyslov_variation;
pub use smyslov_variation::SMYSLOV_VARIATION;
pub mod soultanbeieff_variation;
pub use soultanbeieff_variation::SOULTANBEIEFF_VARIATION;
pub mod czech_variation;
pub use czech_variation::CZECH_VARIATION;
pub mod steiner_variation;
pub use steiner_variation::STEINER_VARIATION;
pub mod slav_gambit;
pub use slav_gambit::SLAV_GAMBIT;
pub mod schlechter_variation;
pub use schlechter_variation::SCHLECHTER_VARIATION;
pub mod modern_line;
pub use modern_line::MODERN_LINE;
pub mod suchting_variation;
pub use suchting_variation::SUCHTING_VARIATION;
pub mod two_knights_attack;
pub use two_knights_attack::TWO_KNIGHTS_ATTACK;
pub mod alekhine_variation;
pub use alekhine_variation::ALEKHINE_VARIATION;
pub mod quiet_variation;
pub use quiet_variation::QUIET_VARIATION;
pub mod three_knights_variation;
pub use three_knights_variation::THREE_KNIGHTS_VARIATION;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
