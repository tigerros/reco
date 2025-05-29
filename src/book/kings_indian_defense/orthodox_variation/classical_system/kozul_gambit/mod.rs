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
# use reco::book::kings_indian_defense::orthodox_variation::classical_system::KOZUL_GAMBIT;
assert_eq!(KOZUL_GAMBIT.original_name(), "King's Indian Defense: Orthodox Variation, Classical System, Kozul Gambit");
```"#)]
pub static KOZUL_GAMBIT: Variation = Variation {
    name: "Kozul Gambit",
    variations: &[],
    parent: Some(&super::CLASSICAL_SYSTEM),
    lines: &[Line {
    code: Code {
        volume: Volume::E,
        category: Category::new_static::<98>()
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
        role: Knight,
        from: G1,
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
        from: F1,
        capture: None,
        to: E2,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E7,
        capture: None,
        to: E5,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
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
        from: D4,
        capture: None,
        to: D5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: C6,
        capture: None,
        to: E7,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F3,
        capture: None,
        to: E1,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F6,
        capture: None,
        to: D7,
        promotion: None,
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
        from: F7,
        capture: None,
        to: F5,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: F2,
        capture: None,
        to: F3,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: F5,
        capture: None,
        to: F4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: E3,
        capture: None,
        to: F2,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: G6,
        capture: None,
        to: G5,
        promotion: None,
    },
    Normal {
        role: Rook,
        from: A1,
        capture: None,
        to: C1,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: E7,
        capture: None,
        to: G6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C4,
        capture: None,
        to: C5,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(38008313893405440),
                knight: Bitboard(2322168558125072),
                bishop: Bitboard(306244774661206016),
                rook: Bitboard(2377900603251621924),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387968)
            },
            ByColor {
                black: Bitboard(7912622579284705280),
                white: Bitboard(51810464636)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(14) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};