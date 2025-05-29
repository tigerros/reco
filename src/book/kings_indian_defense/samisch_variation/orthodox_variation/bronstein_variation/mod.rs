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
# use reco::book::kings_indian_defense::samisch_variation::orthodox_variation::BRONSTEIN_VARIATION;
assert_eq!(BRONSTEIN_VARIATION.original_name(), "King's Indian Defense: Sämisch Variation, Orthodox Variation, Bronstein Variation");
```"#)]
pub static BRONSTEIN_VARIATION: Variation = Variation {
    name: "Bronstein Variation",
    variations: &[],
    parent: Some(&super::ORTHODOX_VARIATION),
    lines: &[Line {
    code: Code {
        volume: Volume::E,
        category: Category::new_static::<87>()
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
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
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
        from: G7,
        capture: None,
        to: G6,
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
        role: Bishop,
        from: F8,
        capture: None,
        to: G7,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E2,
        capture: None,
        to: E4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D7,
        capture: None,
        to: D6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: F2,
        capture: None,
        to: F3,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
    },
    Normal {
        role: Bishop,
        from: C1,
        capture: None,
        to: E3,
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
        from: D4,
        capture: None,
        to: D5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F6,
        capture: None,
        to: H5,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D1,
        capture: None,
        to: D2,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D8,
        capture: None,
        to: H4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: G2,
        capture: None,
        to: G3,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: H5,
        capture: Some(
            Pawn,
        ),
        to: G3,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D2,
        capture: None,
        to: F2,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G3,
        capture: Some(
            Bishop,
        ),
        to: F1,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: F2,
        capture: Some(
            Queen,
        ),
        to: H4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F1,
        capture: Some(
            Bishop,
        ),
        to: E3,
        promotion: None,
    },
    Normal {
        role: King,
        from: E1,
        capture: None,
        to: E2,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: E3,
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
                pawn: Bitboard(47085589297660672),
                knight: Bitboard(144115188143226944),
                bishop: Bitboard(306244774661193728),
                rook: Bitboard(2377900603251622017),
                queen: Bitboard(2147483648),
                king: Bitboard(4611686018427392000)
            },
            ByColor {
                black: Bitboard(7487032139150524416),
                white: Bitboard(36778054593)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(13) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};