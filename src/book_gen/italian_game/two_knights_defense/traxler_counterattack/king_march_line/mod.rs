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
# use reco::book::italian_game::two_knights_defense::traxler_counterattack::KING_MARCH_LINE;
assert_eq!(KING_MARCH_LINE.original_name(), "Italian Game: Two Knights Defense, Traxler Counterattack, King March Line");
```"#
)]
pub static KING_MARCH_LINE: Variation = Variation {
    name: "King March Line",
    parent: Some(&super::TRAXLER_COUNTERATTACK),
    variations: &[],
    lines: &[Line {
        parent: &KING_MARCH_LINE,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<57>(),
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
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: None,
                to: G5,
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
                role: Knight,
                from: G5,
                capture: Some(Pawn),
                to: F7,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: C5,
                capture: Some(Pawn),
                to: F2,
                promotion: None,
            },
            Normal {
                role: King,
                from: E1,
                capture: Some(Bishop),
                to: F2,
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
                role: King,
                from: F2,
                capture: None,
                to: E3,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(58265388898635520),
                    knight: Bitboard(9011597569687554),
                    bishop: Bitboard(288230376218820612),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504607895552),
                },
                ByColor {
                    black: Bitboard(11371312051168215040),
                    white: Bitboard(9007199322951567),
                },
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
