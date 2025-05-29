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
# use reco::book::NIMZO_INDIAN_DEFENSE;
assert_eq!(NIMZO_INDIAN_DEFENSE.original_name(), "Nimzo-Indian Defense");
```"#)]
pub static NIMZO_INDIAN_DEFENSE: Variation = Variation {
    name: "Nimzo-Indian Defense",
    variations: &[&KMOCH_VARIATION,
&SPIELMANN_VARIATION,
&RAGOZIN_DEFENSE,
&LENINGRAD_VARIATION,
&RUBINSTEIN_SYSTEM,
&ST_PETERSBURG_VARIATION,
&DILWORTH_GAMBIT,
&CLASSICAL_VARIATION,
&RAGOZIN_VARIATION,
&RESHEVSKY_VARIATION,
&SIMAGIN_VARIATION,
&SAMISCH_VARIATION,
&NORMAL_VARIATION,
&MIKENAS_ATTACK,
&ROMANISHIN_VARIATION,
&THREE_KNIGHTS_VARIATION],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::E,
        category: Category::new_static::<20>()
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
        role: Bishop,
        from: F8,
        capture: None,
        to: B4,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67290111821280000),
                knight: Bitboard(144150372448206912),
                bishop: Bitboard(288230376185266212),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(11524482748056076288),
                white: Bitboard(201651197)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 2,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
},
Line {
    code: Code {
        volume: Volume::E,
        category: Category::new_static::<50>()
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
        role: Bishop,
        from: F8,
        capture: None,
        to: B4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E2,
        capture: None,
        to: E3,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
    },
    Normal {
        role: Knight,
        from: G1,
        capture: None,
        to: F3,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67290111822324480),
                knight: Bitboard(144150372450304000),
                bishop: Bitboard(288230376185266212),
                rook: Bitboard(2377900603251622017),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387920)
            },
            ByColor {
                black: Bitboard(8065718234235535360),
                white: Bitboard(204792765)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(129),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 2,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod kmoch_variation;
pub use kmoch_variation::KMOCH_VARIATION;
pub mod spielmann_variation;
pub use spielmann_variation::SPIELMANN_VARIATION;
pub mod ragozin_defense;
pub use ragozin_defense::RAGOZIN_DEFENSE;
pub mod leningrad_variation;
pub use leningrad_variation::LENINGRAD_VARIATION;
pub mod rubinstein_system;
pub use rubinstein_system::RUBINSTEIN_SYSTEM;
pub mod st_petersburg_variation;
pub use st_petersburg_variation::ST_PETERSBURG_VARIATION;
pub mod dilworth_gambit;
pub use dilworth_gambit::DILWORTH_GAMBIT;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod ragozin_variation;
pub use ragozin_variation::RAGOZIN_VARIATION;
pub mod reshevsky_variation;
pub use reshevsky_variation::RESHEVSKY_VARIATION;
pub mod simagin_variation;
pub use simagin_variation::SIMAGIN_VARIATION;
pub mod samisch_variation;
pub use samisch_variation::SAMISCH_VARIATION;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod mikenas_attack;
pub use mikenas_attack::MIKENAS_ATTACK;
pub mod romanishin_variation;
pub use romanishin_variation::ROMANISHIN_VARIATION;
pub mod three_knights_variation;
pub use three_knights_variation::THREE_KNIGHTS_VARIATION;
