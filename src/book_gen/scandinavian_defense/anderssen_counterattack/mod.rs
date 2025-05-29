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
# use reco::book::scandinavian_defense::ANDERSSEN_COUNTERATTACK;
assert_eq!(ANDERSSEN_COUNTERATTACK.original_name(), "Scandinavian Defense: Anderssen Counterattack");
```"#
)]
pub static ANDERSSEN_COUNTERATTACK: Variation = Variation {
    name: "Anderssen Counterattack",
    parent: Some(&super::SCANDINAVIAN_DEFENSE),
    variations: &[&ORTHODOX_ATTACK, &COLLIJN_VARIATION, &GOTEBORG_SYSTEM],
    lines: &[Line {
        parent: &ANDERSSEN_COUNTERATTACK,
        code: Code {
            volume: Volume::B,
            category: Category::new_static::<1>(),
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
                from: D7,
                capture: None,
                to: D5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E4,
                capture: Some(Pawn),
                to: D5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D8,
                capture: Some(Pawn),
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
                role: Queen,
                from: D5,
                capture: None,
                to: A5,
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
                from: E7,
                capture: None,
                to: E5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65020788473915136),
                    knight: Bitboard(4755801206503505984),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(4294967304),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(17863246520002805760),
                    white: Bitboard(134539261),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
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
pub mod orthodox_attack;
pub use orthodox_attack::ORTHODOX_ATTACK;
pub mod collijn_variation;
pub use collijn_variation::COLLIJN_VARIATION;
pub mod goteborg_system;
pub use goteborg_system::GOTEBORG_SYSTEM;
