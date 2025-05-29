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
# use reco::book::ruy_lopez::CLOSED;
assert_eq!(CLOSED.original_name(), "Ruy Lopez: Closed");
```"#)]
pub static CLOSED: Variation = Variation {
    name: "Closed",
    variations: &[&AVERBAKH_VARIATION,
&MARTINEZ_VARIATION,
&BORISENKO_VARIATION,
&MORPHY_ATTACK,
&ROSEN_ATTACK,
&SMYSLOV_DEFENSE,
&KERES_DEFENSE,
&PILNIK_VARIATION,
&SMYSLOV_BREYER_ZAITSEV_HYBRID,
&CHIGORIN_DEFENSE,
&BASQUE_GAMBIT,
&KHOLMOV_VARIATION,
&LUTIKOV_VARIATION,
&BALLA_VARIATION,
&SUETIN_VARIATION,
&ROSSOLIMO_DEFENSE,
&KECSKEMET_VARIATION,
&ZAITSEV_SYSTEM,
&CHIGORIN,
&WORRALL_ATTACK,
&ANTI_MARSHALL,
&TRAJKOVIC_COUNTERATTACK,
&BREYER_DEFENSE,
&DELAYED_EXCHANGE,
&ALEKHINE_GAMBIT,
&KARPOV_VARIATION,
&FLOHR_SYSTEM,
&LEONHARDT_VARIATION,
&YATES_VARIATION,
&BREYER,
&BOGOLJUBOW_VARIATION,
&CENTER_ATTACK],
    parent: Some(&super::RUY_LOPEZ),
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<88>()
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
        capture: None,
        to: A4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: E7,
        promotion: None,
    },
    Normal {
        role: Rook,
        from: F1,
        capture: None,
        to: E1,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B7,
        capture: None,
        to: B5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: A4,
        capture: None,
        to: B3,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(66429271593250560),
                knight: Bitboard(39582420697090),
                bishop: Bitboard(292733975779213316),
                rook: Bitboard(9295429630892703761),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606847040)
            },
            ByColor {
                black: Bitboard(11384014717325410304),
                white: Bitboard(270724959)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(9295429630892703744),
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
},
Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<88>()
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
        capture: None,
        to: A4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: E7,
        promotion: None,
    },
    Normal {
        role: Rook,
        from: F1,
        capture: None,
        to: E1,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B7,
        capture: None,
        to: B5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: A4,
        capture: None,
        to: B3,
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
                pawn: Bitboard(66429271593250560),
                knight: Bitboard(39582420697090),
                bishop: Bitboard(292733975779213316),
                rook: Bitboard(2377900603251621905),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387968)
            },
            ByColor {
                black: Bitboard(7925250203504869376),
                white: Bitboard(270724959)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 2,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
},
Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<90>()
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
        capture: None,
        to: A4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: E7,
        promotion: None,
    },
    Normal {
        role: Rook,
        from: F1,
        capture: None,
        to: E1,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B7,
        capture: None,
        to: B5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: A4,
        capture: None,
        to: B3,
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
        from: C2,
        capture: None,
        to: C3,
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
                pawn: Bitboard(64186267872848640),
                knight: Bitboard(39582420697090),
                bishop: Bitboard(292733975779213316),
                rook: Bitboard(2377900603251621905),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387968)
            },
            ByColor {
                black: Bitboard(7923007199784206336),
                white: Bitboard(270986079)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
},
Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<92>()
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
        capture: None,
        to: A4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: E7,
        promotion: None,
    },
    Normal {
        role: Rook,
        from: F1,
        capture: None,
        to: E1,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B7,
        capture: None,
        to: B5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: A4,
        capture: None,
        to: B3,
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
        from: C2,
        capture: None,
        to: C3,
        promotion: None,
    },
    Castle {
        king: E8,
        rook: H8,
    },
    Normal {
        role: Pawn,
        from: H2,
        capture: None,
        to: H3,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(64186267881204480),
                knight: Bitboard(39582420697090),
                bishop: Bitboard(292733975779213316),
                rook: Bitboard(2377900603251621905),
                queen: Bitboard(576460752303423496),
                king: Bitboard(4611686018427387968)
            },
            ByColor {
                black: Bitboard(7923007199784206336),
                white: Bitboard(279341919)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
        castling_rights: Bitboard(0),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
},
Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<84>()
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
        capture: None,
        to: A4,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
    Castle {
        king: E1,
        rook: H1,
    },
    Normal {
        role: Bishop,
        from: F8,
        capture: None,
        to: E7,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(66992212956737280),
                knight: Bitboard(39582420697090),
                bishop: Bitboard(292733975795859460),
                rook: Bitboard(9295429630892703777),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606847040)
            },
            ByColor {
                black: Bitboard(11384577658688897024),
                white: Bitboard(287371119)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703744),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 4,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod averbakh_variation;
pub use averbakh_variation::AVERBAKH_VARIATION;
pub mod martinez_variation;
pub use martinez_variation::MARTINEZ_VARIATION;
pub mod borisenko_variation;
pub use borisenko_variation::BORISENKO_VARIATION;
pub mod morphy_attack;
pub use morphy_attack::MORPHY_ATTACK;
pub mod rosen_attack;
pub use rosen_attack::ROSEN_ATTACK;
pub mod smyslov_defense;
pub use smyslov_defense::SMYSLOV_DEFENSE;
pub mod keres_defense;
pub use keres_defense::KERES_DEFENSE;
pub mod pilnik_variation;
pub use pilnik_variation::PILNIK_VARIATION;
pub mod smyslov_breyer_zaitsev_hybrid;
pub use smyslov_breyer_zaitsev_hybrid::SMYSLOV_BREYER_ZAITSEV_HYBRID;
pub mod chigorin_defense;
pub use chigorin_defense::CHIGORIN_DEFENSE;
pub mod basque_gambit;
pub use basque_gambit::BASQUE_GAMBIT;
pub mod kholmov_variation;
pub use kholmov_variation::KHOLMOV_VARIATION;
pub mod lutikov_variation;
pub use lutikov_variation::LUTIKOV_VARIATION;
pub mod balla_variation;
pub use balla_variation::BALLA_VARIATION;
pub mod suetin_variation;
pub use suetin_variation::SUETIN_VARIATION;
pub mod rossolimo_defense;
pub use rossolimo_defense::ROSSOLIMO_DEFENSE;
pub mod kecskemet_variation;
pub use kecskemet_variation::KECSKEMET_VARIATION;
pub mod zaitsev_system;
pub use zaitsev_system::ZAITSEV_SYSTEM;
pub mod chigorin;
pub use chigorin::CHIGORIN;
pub mod worrall_attack;
pub use worrall_attack::WORRALL_ATTACK;
pub mod anti_marshall;
pub use anti_marshall::ANTI_MARSHALL;
pub mod trajkovic_counterattack;
pub use trajkovic_counterattack::TRAJKOVIC_COUNTERATTACK;
pub mod breyer_defense;
pub use breyer_defense::BREYER_DEFENSE;
pub mod delayed_exchange;
pub use delayed_exchange::DELAYED_EXCHANGE;
pub mod alekhine_gambit;
pub use alekhine_gambit::ALEKHINE_GAMBIT;
pub mod karpov_variation;
pub use karpov_variation::KARPOV_VARIATION;
pub mod flohr_system;
pub use flohr_system::FLOHR_SYSTEM;
pub mod leonhardt_variation;
pub use leonhardt_variation::LEONHARDT_VARIATION;
pub mod yates_variation;
pub use yates_variation::YATES_VARIATION;
pub mod breyer;
pub use breyer::BREYER;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod center_attack;
pub use center_attack::CENTER_ATTACK;
