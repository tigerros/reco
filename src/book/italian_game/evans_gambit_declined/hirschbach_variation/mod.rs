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
# use reco::book::italian_game::evans_gambit_declined::HIRSCHBACH_VARIATION;
assert_eq!(HIRSCHBACH_VARIATION.original_name(), "Italian Game: Evans Gambit Declined, Hirschbach Variation");
```"#)]
pub static HIRSCHBACH_VARIATION: Variation = Variation {
    name: "Hirschbach Variation",
    variations: &[],
    parent: Some(&super::EVANS_GAMBIT_DECLINED),
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<51>()
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
        to: C4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: C5,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B2,
        capture: None,
        to: B4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C5,
        capture: None,
        to: B6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B4,
        capture: None,
        to: B5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: C6,
        capture: None,
        to: A5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F3,
        capture: Some(
            Pawn,
        ),
        to: E5,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D8,
        capture: None,
        to: G5,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272528292277504),
                knight: Bitboard(4611686091441831938),
                bishop: Bitboard(288232575242076164),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(274877906952),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(15415542527708626944),
                white: Bitboard(77645016479)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};