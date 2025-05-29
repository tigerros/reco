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
# use reco::book::ruy_lopez::berlin_defense::l_hermet_variation::BERLIN_WALL_DEFENSE;
assert_eq!(BERLIN_WALL_DEFENSE.original_name(), "Ruy Lopez: Berlin Defense, l'Hermet Variation, Berlin Wall Defense");
```"#)]
pub static BERLIN_WALL_DEFENSE: Variation = Variation {
    name: "Berlin Wall Defense",
    variations: &[],
    parent: Some(&super::L_HERMET_VARIATION),
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<67>()
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
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
    },
    Normal {
        role: Knight,
        from: F6,
        capture: Some(
            Pawn,
        ),
        to: E4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: E4,
        capture: None,
        to: D6,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: B5,
        capture: Some(
            Knight,
        ),
        to: C6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D7,
        capture: Some(
            Bishop,
        ),
        to: C6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D4,
        capture: Some(
            Pawn,
        ),
        to: E5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: D6,
        capture: None,
        to: F5,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D1,
        capture: Some(
            Queen,
        ),
        to: D8,
        promotion: None,
    },
    Normal {
        role: King,
        from: E8,
        capture: Some(
            Queen,
        ),
        to: D8,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(65025186386208512),
                knight: Bitboard(137441050626),
                bishop: Bitboard(2594073385365405700),
                rook: Bitboard(9295429630892703777),
                queen: Bitboard(0),
                king: Bitboard(576460752303423552)
            },
            ByColor {
                black: Bitboard(12530989023667159040),
                white: Bitboard(68721633127)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};