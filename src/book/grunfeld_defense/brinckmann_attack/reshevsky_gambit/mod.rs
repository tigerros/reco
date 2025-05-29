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
# use reco::book::grunfeld_defense::brinckmann_attack::RESHEVSKY_GAMBIT;
assert_eq!(RESHEVSKY_GAMBIT.original_name(), "Grünfeld Defense: Brinckmann Attack, Reshevsky Gambit");
```"#)]
pub static RESHEVSKY_GAMBIT: Variation = Variation {
    name: "Reshevsky Gambit",
    variations: &[],
    parent: Some(&super::BRINCKMANN_ATTACK),
    lines: &[Line {
    code: Code {
        volume: Volume::D,
        category: Category::new_static::<83>()
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
        role: Pawn,
        from: D7,
        capture: None,
        to: D5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C1,
        capture: None,
        to: F4,
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
        role: Rook,
        from: A1,
        capture: None,
        to: C1,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
    },
    Normal {
        role: Pawn,
        from: E2,
        capture: None,
        to: E3,
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
        from: D4,
        capture: Some(
            Pawn,
        ),
        to: C5,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D8,
        capture: None,
        to: A5,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(50454441183208192),
                knight: Bitboard(144150372448206912),
                bishop: Bitboard(306244775198064672),
                rook: Bitboard(2377900603251622020),
                queen: Bitboard(4294967304),
                king: Bitboard(4611686018427387920)
            },
            ByColor {
                black: Bitboard(7490436197018238976),
                white: Bitboard(17785218044)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(128),
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