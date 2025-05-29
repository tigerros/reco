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
# use reco::book::english_opening::ANGLO_INDIAN_DEFENSE;
assert_eq!(ANGLO_INDIAN_DEFENSE.original_name(), "English Opening: Anglo-Indian Defense");
```"#)]
pub static ANGLO_INDIAN_DEFENSE: Variation = Variation {
    name: "Anglo-Indian Defense",
    variations: &[&SCANDINAVIAN_DEFENSE,
&GRUNFELD_FORMATION,
&ANTI_ANTI_GRUNFELD,
&ROMANISHIN_VARIATION,
&HEDGEHOG_SYSTEM,
&NIMZO_ENGLISH,
&QUEENS_INDIAN_FORMATION,
&SLAV_FORMATION,
&QUEENS_INDIAN_VARIATION,
&KINGS_KNIGHT_VARIATION,
&OLD_INDIAN_FORMATION,
&ANGLO_GRUNFELD_VARIATION,
&QUEENS_KNIGHT_VARIATION,
&ZVJAGINSEV_KRASENKOW_ATTACK,
&FLOHR_MIKENAS_CARLS_VARIATION,
&KINGS_INDIAN_FORMATION],
    parent: Some(&super::ENGLISH_OPENING),
    lines: &[Line {
    code: Code {
        volume: Volume::A,
        category: Category::new_static::<17>()
    },
    moves: &[
    Normal {
        role: Pawn,
        from: C2,
        capture: None,
        to: C4,
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
        role: Pawn,
        from: C4,
        capture: Some(
            Pawn,
        ),
        to: D5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: F6,
        capture: Some(
            Pawn,
        ),
        to: D5,
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
        role: Pawn,
        from: C7,
        capture: None,
        to: C5,
        promotion: None,
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
        from: E7,
        capture: None,
        to: E6,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(63912429080341248),
                knight: Bitboard(144115222437953536),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13826912924683206656),
                white: Bitboard(3468221)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
},
Line {
    code: Code {
        volume: Volume::A,
        category: Category::new_static::<15>()
    },
    moves: &[
    Normal {
        role: Pawn,
        from: C2,
        capture: None,
        to: C4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(71776119128390400),
                knight: Bitboard(144150372447944770),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13834811764677541888),
                white: Bitboard(67173375)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod scandinavian_defense;
pub use scandinavian_defense::SCANDINAVIAN_DEFENSE;
pub mod grunfeld_formation;
pub use grunfeld_formation::GRUNFELD_FORMATION;
pub mod anti_anti_grunfeld;
pub use anti_anti_grunfeld::ANTI_ANTI_GRUNFELD;
pub mod romanishin_variation;
pub use romanishin_variation::ROMANISHIN_VARIATION;
pub mod hedgehog_system;
pub use hedgehog_system::HEDGEHOG_SYSTEM;
pub mod nimzo_english;
pub use nimzo_english::NIMZO_ENGLISH;
pub mod queens_indian_formation;
pub use queens_indian_formation::QUEENS_INDIAN_FORMATION;
pub mod slav_formation;
pub use slav_formation::SLAV_FORMATION;
pub mod queens_indian_variation;
pub use queens_indian_variation::QUEENS_INDIAN_VARIATION;
pub mod kings_knight_variation;
pub use kings_knight_variation::KINGS_KNIGHT_VARIATION;
pub mod old_indian_formation;
pub use old_indian_formation::OLD_INDIAN_FORMATION;
pub mod anglo_grunfeld_variation;
pub use anglo_grunfeld_variation::ANGLO_GRUNFELD_VARIATION;
pub mod queens_knight_variation;
pub use queens_knight_variation::QUEENS_KNIGHT_VARIATION;
pub mod zvjaginsev_krasenkow_attack;
pub use zvjaginsev_krasenkow_attack::ZVJAGINSEV_KRASENKOW_ATTACK;
pub mod flohr_mikenas_carls_variation;
pub use flohr_mikenas_carls_variation::FLOHR_MIKENAS_CARLS_VARIATION;
pub mod kings_indian_formation;
pub use kings_indian_formation::KINGS_INDIAN_FORMATION;
