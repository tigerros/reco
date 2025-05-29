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
# use reco::book::french_defense::winawer_variation::poisoned_pawn_variation::MAIN_LINE;
assert_eq!(MAIN_LINE.original_name(), "French Defense: Winawer Variation, Poisoned Pawn Variation, Main Line");
```"#)]
pub static MAIN_LINE: Variation = Variation {
    name: "Main Line",
    variations: &[],
    parent: Some(&super::POISONED_PAWN_VARIATION),
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<19>()
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
        to: E6,
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
        from: D7,
        capture: None,
        to: D5,
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
        to: B4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E4,
        capture: None,
        to: E5,
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
        role: Pawn,
        from: A2,
        capture: None,
        to: A3,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: B4,
        capture: Some(
            Knight,
        ),
        to: C3,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B2,
        capture: Some(
            Bishop,
        ),
        to: C3,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: E7,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D1,
        capture: None,
        to: G4,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D8,
        capture: None,
        to: C7,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: G4,
        capture: Some(
            Pawn,
        ),
        to: G7,
        promotion: None,
    },
    Normal {
        role: Rook,
        from: H8,
        capture: None,
        to: G8,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: G7,
        capture: Some(
            Pawn,
        ),
        to: H7,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C5,
        capture: Some(
            Pawn,
        ),
        to: D4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G1,
        capture: None,
        to: E2,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(9869319584736256),
                knight: Bitboard(148618787703230464),
                bishop: Bitboard(288230376151711780),
                rook: Bitboard(4683743612465315969),
                queen: Bitboard(37154696925806592),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(6284509431698817024),
                white: Bitboard(36028865738831029)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(72057594037928065),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};