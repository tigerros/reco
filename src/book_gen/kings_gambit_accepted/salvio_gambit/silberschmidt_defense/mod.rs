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
# use reco::book::kings_gambit_accepted::salvio_gambit::SILBERSCHMIDT_DEFENSE;
assert_eq!(SILBERSCHMIDT_DEFENSE.original_name(), "King's Gambit Accepted: Salvio Gambit, Silberschmidt Defense");
```"#
)]
pub static SILBERSCHMIDT_DEFENSE: Variation = Variation {
    name: "Silberschmidt Defense",
    parent: Some(&super::SALVIO_GAMBIT),
    variations: &[],
    lines: &[Line {
        parent: &SILBERSCHMIDT_DEFENSE,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<37>(),
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
                role: Pawn,
                from: F2,
                capture: None,
                to: F4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E5,
                capture: Some(Pawn),
                to: F4,
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
                from: G7,
                capture: None,
                to: G5,
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
                role: Pawn,
                from: G5,
                capture: None,
                to: G4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: None,
                to: E5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D8,
                capture: None,
                to: H4,
                promotion: None,
            },
            Normal {
                role: King,
                from: E1,
                capture: None,
                to: F1,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: H6,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(49258122803465984),
                    knight: Bitboard(144255994283687938),
                    bishop: Bitboard(2594073385432514564),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(2147483656),
                    king: Bitboard(1152921504606847008),
                },
                ByColor {
                    black: Bitboard(13235938571111628800),
                    white: Bitboard(69055074223),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703744),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 4,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
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
