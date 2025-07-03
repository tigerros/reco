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
use core::unreachable;
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
# use reco::book::grunfeld_defense::brinckmann_attack::GRUNFELD_GAMBIT;
assert_eq!(GRUNFELD_GAMBIT.original_name(), "Grünfeld Defense: Brinckmann Attack, Grünfeld Gambit");
```"#
)]
pub static GRUNFELD_GAMBIT: Variation = Variation {
    name: "Grünfeld Gambit",
    parent: Some(&super::BRINCKMANN_ATTACK),
    variations: &[&BOTVINNIK_VARIATION, &CAPABLANCA_VARIATION],
    lines: &[Line {
        parent: &GRUNFELD_GAMBIT,
        code: Code {
            volume: Volume::D,
            category: Category(RangedU8::new_static::<8>()),
        },
        moves: &[
            Normal {
                role: Pawn,
                from: Square::D2,
                capture: None,
                to: Square::D4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::G8,
                capture: None,
                to: Square::F6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C2,
                capture: None,
                to: Square::C4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::G7,
                capture: None,
                to: Square::G6,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::B1,
                capture: None,
                to: Square::C3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::D7,
                capture: None,
                to: Square::D5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C1,
                capture: None,
                to: Square::F4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F8,
                capture: None,
                to: Square::G7,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E2,
                capture: None,
                to: Square::E3,
                promotion: None,
            },
            Castle {
                king: Square::E8,
                rook: Square::H8,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(51580324044399360),
                    knight: Bitboard(144150372448206912),
                    bishop: Bitboard(306244775198064672),
                    rook: Bitboard(2377900603251622017),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387920),
                },
                ByColor {
                    black: Bitboard(8068022844933537792),
                    white: Bitboard(739566585),
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
            castling_rights: Bitboard(129),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
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
pub mod botvinnik_variation;
pub use botvinnik_variation::BOTVINNIK_VARIATION;
pub mod capablanca_variation;
pub use capablanca_variation::CAPABLANCA_VARIATION;
