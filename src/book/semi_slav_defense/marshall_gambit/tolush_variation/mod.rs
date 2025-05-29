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
# use reco::book::semi_slav_defense::marshall_gambit::TOLUSH_VARIATION;
assert_eq!(TOLUSH_VARIATION.original_name(), "Semi-Slav Defense: Marshall Gambit, Tolush Variation");
```"#)]
pub static TOLUSH_VARIATION: Variation = Variation {
    name: "Tolush Variation",
    variations: &[],
    parent: Some(&super::MARSHALL_GAMBIT),
    lines: &[Line {
    code: Code {
        volume: Volume::D,
        category: Category::new_static::<31>()
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
        to: C6,
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
        from: D5,
        capture: Some(
            Pawn,
        ),
        to: E4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: C3,
        capture: Some(
            Pawn,
        ),
        to: E4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: B4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C1,
        capture: None,
        to: D2,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D8,
        capture: Some(
            Pawn,
        ),
        to: D4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: D2,
        capture: Some(
            Bishop,
        ),
        to: B4,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D4,
        capture: Some(
            Knight,
        ),
        to: E4,
        promotion: None,
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
        from: C6,
        capture: None,
        to: C5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: B4,
        capture: Some(
            Pawn,
        ),
        to: C5,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: E4,
        capture: Some(
            Pawn,
        ),
        to: G2,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63912411966513920),
                knight: Bitboard(4755801206503243840),
                bishop: Bitboard(288230393331585024),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(16392),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(15556295130053885952),
                white: Bitboard(17247024089)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};