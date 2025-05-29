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
# use reco::book::ruy_lopez::exchange::ALEKHINE_VARIATION;
assert_eq!(ALEKHINE_VARIATION.original_name(), "Ruy Lopez: Exchange, Alekhine Variation");
```"#)]
pub static ALEKHINE_VARIATION: Variation = Variation {
    name: "Alekhine Variation",
    variations: &[],
    parent: Some(&super::EXCHANGE),
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<68>()
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
        from: A7,
        capture: None,
        to: A6,
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
        from: D2,
        capture: None,
        to: D4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E5,
        capture: Some(
            Pawn,
        ),
        to: D4,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D1,
        capture: Some(
            Pawn,
        ),
        to: D4,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D8,
        capture: Some(
            Queen,
        ),
        to: D4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F3,
        capture: Some(
            Queen,
        ),
        to: D4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C8,
        capture: None,
        to: D7,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(64744742470084352),
                knight: Bitboard(4611686018561605634),
                bishop: Bitboard(2308094809027379204),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(0),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(17432876705155907584),
                white: Bitboard(402712471)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};