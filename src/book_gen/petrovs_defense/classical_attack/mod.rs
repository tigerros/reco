#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use crate::{Category, Code, Line, Variation, Volume};
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use core::num::NonZeroU32;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Color::{Black, White};
#[allow(
    unused_imports,
    clippy::enum_glob_use,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Move::*;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Role::{Bishop, King, Knight, Pawn, Queen, Rook};
#[allow(
    clippy::enum_glob_use,
    reason = "there's 64 variants in this enum, importing them all is stupid"
)]
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square::*;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::bitboard::Bitboard;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::board::Board;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::{ByColor, ByRole, Setup};
#[cfg_attr(
    feature = "alloc",
    doc = r#"```rust
# use reco::book::petrovs_defense::CLASSICAL_ATTACK;
assert_eq!(CLASSICAL_ATTACK.original_name(), "Petrov's Defense: Classical Attack");
```"#
)]
pub static CLASSICAL_ATTACK: Variation = Variation {
    name: "Classical Attack",
    parent: Some(&super::PETROVS_DEFENSE),
    variations: &[
        &BERGER_VARIATION,
        &CHIGORIN_VARIATION,
        &CLOSED_VARIATION,
        &JAENISCH_VARIATION,
        &KRAUSE_VARIATION,
        &MARSHALL_TRAP,
        &MARSHALL_VARIATION,
        &MAROCZY_VARIATION,
        &MASON_VARIATION,
        &MASON_SHOWALTER_VARIATION,
        &STAUNTON_VARIATION,
        &TARRASCH_VARIATION,
    ],
    lines: &[Line {
        parent: &CLASSICAL_ATTACK,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<42>(),
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
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: Some(Pawn),
                to: E5,
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
                role: Knight,
                from: E5,
                capture: None,
                to: F3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F6,
                capture: Some(Pawn),
                to: E4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D2,
                capture: None,
                to: D4,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65029515847460608),
                    knight: Bitboard(144115188346388482),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13828029977225854976),
                    white: Bitboard(136374207),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
                fullmoves
            } else {
                #[expect(
                    clippy::unreachable,
                    reason = "intentional. It's in a const expression"
                )]
                {
                    unreachable!()
                }
            },
        },
    }],
};
pub mod berger_variation;
pub use berger_variation::BERGER_VARIATION;
pub mod chigorin_variation;
pub use chigorin_variation::CHIGORIN_VARIATION;
pub mod closed_variation;
pub use closed_variation::CLOSED_VARIATION;
pub mod jaenisch_variation;
pub use jaenisch_variation::JAENISCH_VARIATION;
pub mod krause_variation;
pub use krause_variation::KRAUSE_VARIATION;
pub mod marshall_trap;
pub use marshall_trap::MARSHALL_TRAP;
pub mod marshall_variation;
pub use marshall_variation::MARSHALL_VARIATION;
pub mod maroczy_variation;
pub use maroczy_variation::MAROCZY_VARIATION;
pub mod mason_variation;
pub use mason_variation::MASON_VARIATION;
pub mod mason_showalter_variation;
pub use mason_showalter_variation::MASON_SHOWALTER_VARIATION;
pub mod staunton_variation;
pub use staunton_variation::STAUNTON_VARIATION;
pub mod tarrasch_variation;
pub use tarrasch_variation::TARRASCH_VARIATION;
