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
# use reco::book::english_opening::SYMMETRICAL_VARIATION;
assert_eq!(SYMMETRICAL_VARIATION.original_name(), "English Opening: Symmetrical Variation");
```"#)]
pub static SYMMETRICAL_VARIATION: Variation = Variation {
    name: "Symmetrical Variation",
    variations: &[&NAPOLITANO_GAMBIT,
&FIANCHETTO_VARIATION,
&ANTI_BENONI_VARIATION,
&RUBINSTEIN_VARIATION,
&DOUBLE_FIANCHETTO,
&FOUR_KNIGHTS_VARIATION,
&THREE_KNIGHTS_VARIATION,
&FULL_SYMMETRY_LINE,
&MECKING_VARIATION,
&NORMAL_VARIATION,
&BOTVINNIK_SYSTEM_REVERSED,
&TWO_KNIGHTS,
&TWO_KNIGHTS_VARIATION,
&THREE_KNIGHTS,
&ULTRA_SYMMETRICAL_VARIATION,
&DUCHAMP_VARIATION,
&HEDGEHOG_DEFENSE,
&BOTVINNIK_SYSTEM],
    parent: Some(&super::ENGLISH_OPENING),
    lines: &[Line {
    code: Code {
        volume: Volume::A,
        category: Category::new_static::<30>()
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
        role: Pawn,
        from: C7,
        capture: None,
        to: C5,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(70650236401416960),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18445336716005867520),
                white: Bitboard(67173375)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
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
        category: Category::new_static::<30>()
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
        role: Pawn,
        from: C7,
        capture: None,
        to: C5,
        promotion: None,
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
                pawn: Bitboard(70650236401416960),
                knight: Bitboard(4755801206505340930),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18445336716005867520),
                white: Bitboard(69270463)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: Black,
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
},
Line {
    code: Code {
        volume: Volume::A,
        category: Category::new_static::<34>()
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
        role: Pawn,
        from: C7,
        capture: None,
        to: C5,
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
        role: Knight,
        from: B8,
        capture: None,
        to: C6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: G2,
        capture: None,
        to: G3,
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
        from: D2,
        capture: None,
        to: D4,
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
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(68398453905994496),
                knight: Bitboard(39582420959232),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13687323309421625344),
                white: Bitboard(73708477)
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
        category: Category::new_static::<35>()
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
        role: Pawn,
        from: C7,
        capture: None,
        to: C5,
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
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
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
        from: E7,
        capture: None,
        to: E5,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(66146705493523200),
                knight: Bitboard(144150372450304000),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(13829182351042674688),
                white: Bitboard(69532605)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 0,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod napolitano_gambit;
pub use napolitano_gambit::NAPOLITANO_GAMBIT;
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
pub mod anti_benoni_variation;
pub use anti_benoni_variation::ANTI_BENONI_VARIATION;
pub mod rubinstein_variation;
pub use rubinstein_variation::RUBINSTEIN_VARIATION;
pub mod double_fianchetto;
pub use double_fianchetto::DOUBLE_FIANCHETTO;
pub mod four_knights_variation;
pub use four_knights_variation::FOUR_KNIGHTS_VARIATION;
pub mod three_knights_variation;
pub use three_knights_variation::THREE_KNIGHTS_VARIATION;
pub mod full_symmetry_line;
pub use full_symmetry_line::FULL_SYMMETRY_LINE;
pub mod mecking_variation;
pub use mecking_variation::MECKING_VARIATION;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod botvinnik_system_reversed;
pub use botvinnik_system_reversed::BOTVINNIK_SYSTEM_REVERSED;
pub mod two_knights;
pub use two_knights::TWO_KNIGHTS;
pub mod two_knights_variation;
pub use two_knights_variation::TWO_KNIGHTS_VARIATION;
pub mod three_knights;
pub use three_knights::THREE_KNIGHTS;
pub mod ultra_symmetrical_variation;
pub use ultra_symmetrical_variation::ULTRA_SYMMETRICAL_VARIATION;
pub mod duchamp_variation;
pub use duchamp_variation::DUCHAMP_VARIATION;
pub mod hedgehog_defense;
pub use hedgehog_defense::HEDGEHOG_DEFENSE;
pub mod botvinnik_system;
pub use botvinnik_system::BOTVINNIK_SYSTEM;
