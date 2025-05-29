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
# use reco::book::italian_game::classical_variation::greco_gambit::MOELLER_BAYONET_ATTACK;
assert_eq!(MOELLER_BAYONET_ATTACK.original_name(), "Italian Game: Classical Variation, Greco Gambit, Moeller-Bayonet Attack");
```"#
)]
pub static MOELLER_BAYONET_ATTACK: Variation = Variation {
    name: "Moeller-Bayonet Attack",
    parent: Some(&super::GRECO_GAMBIT),
    variations: &[],
    lines: &[Line {
        parent: &MOELLER_BAYONET_ATTACK,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<54>(),
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
                to: C4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: C5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C2,
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
                role: Pawn,
                from: D2,
                capture: None,
                to: D4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E5,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C3,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C5,
                capture: None,
                to: B4,
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
                from: F6,
                capture: Some(Pawn),
                to: E4,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Bishop,
                from: B4,
                capture: Some(Knight),
                to: C3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D4,
                capture: None,
                to: D5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C3,
                capture: None,
                to: F6,
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
                role: Knight,
                from: C6,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: E1,
                capture: Some(Knight),
                to: E4,
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
                from: G2,
                capture: None,
                to: G4,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65029551146705664),
                    knight: Bitboard(4503599629467648),
                    bishop: Bitboard(288265560590909444),
                    rook: Bitboard(9295429631161139201),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606847040),
                },
                ByColor {
                    black: Bitboard(11382610563667329024),
                    white: Bitboard(35771163469),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703744),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(12) {
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
