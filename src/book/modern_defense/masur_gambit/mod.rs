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
# use reco::book::modern_defense::MASUR_GAMBIT;
assert_eq!(MASUR_GAMBIT.original_name(), "Modern Defense: Masur Gambit");
```"#)]
pub static MASUR_GAMBIT: Variation = Variation {
    name: "Masur Gambit",
    variations: &[],
    parent: Some(&super::MODERN_DEFENSE),
    lines: &[Line {
    code: Code {
        volume: Volume::B,
        category: Category::new_static::<6>()
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
        from: G7,
        capture: None,
        to: G6,
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
        from: G8,
        capture: None,
        to: H6,
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
        from: F7,
        capture: None,
        to: F5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C1,
        capture: Some(
            Knight,
        ),
        to: H6,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: Some(
            Bishop,
        ),
        to: H6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E4,
        capture: Some(
            Pawn,
        ),
        to: F5,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(44825027614402304),
                knight: Bitboard(144115188076118080),
                bishop: Bitboard(288371113640067104),
                rook: Bitboard(2377900603251622017),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387920)
            },
            ByColor {
                black: Bitboard(8043358565739528192),
                white: Bitboard(137573492729)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};