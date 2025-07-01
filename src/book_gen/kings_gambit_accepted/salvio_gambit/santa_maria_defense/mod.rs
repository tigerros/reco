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
use deranged::RangedU8;
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
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square;
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
# use reco::book::kings_gambit_accepted::salvio_gambit::SANTA_MARIA_DEFENSE;
assert_eq!(SANTA_MARIA_DEFENSE.original_name(), "King's Gambit Accepted: Salvio Gambit, Santa Maria Defense");
```"#
)]
pub static SANTA_MARIA_DEFENSE: Variation = Variation {
    name: "Santa Maria Defense",
    parent: Some(&super::SALVIO_GAMBIT),
    variations: &[],
    lines: &[Line {
        parent: &SANTA_MARIA_DEFENSE,
        code: Code {
            volume: Volume::C,
            category: Category(RangedU8::new_static::<3>()),
        },
        moves: &[
            Normal {
                role: Pawn,
                from: Square::E2,
                capture: None,
                to: Square::E4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E7,
                capture: None,
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::F2,
                capture: None,
                to: Square::F4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E5,
                capture: Some(Pawn),
                to: Square::F4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::G1,
                capture: None,
                to: Square::F3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::G7,
                capture: None,
                to: Square::G5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F1,
                capture: None,
                to: Square::C4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::G5,
                capture: None,
                to: Square::G4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::F3,
                capture: None,
                to: Square::E5,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D8,
                capture: None,
                to: Square::H4,
                promotion: None,
            },
            Normal {
                role: King,
                from: Square::E1,
                capture: None,
                to: Square::F1,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::G8,
                capture: None,
                to: Square::F6,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(49258122803465984),
                    knight: Bitboard(144150441167421442),
                    bishop: Bitboard(2594073385432514564),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(2147483656),
                    king: Bitboard(1152921504606847008),
                },
                ByColor {
                    black: Bitboard(13235833017995362304),
                    white: Bitboard(69055074223),
                },
            ) {
                board
            } else {
                #[expect(
                    clippy::unreachable,
                    reason = "intentional. It's in a const expression"
                )]
                {
                    unreachable!()
                }
            },
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
