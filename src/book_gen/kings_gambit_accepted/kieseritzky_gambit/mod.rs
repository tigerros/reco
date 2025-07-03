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
# use reco::book::kings_gambit_accepted::KIESERITZKY_GAMBIT;
assert_eq!(KIESERITZKY_GAMBIT.original_name(), "King's Gambit Accepted: Kieseritzky Gambit");
```"#
)]
pub static KIESERITZKY_GAMBIT: Variation = Variation {
    name: "Kieseritzky Gambit",
    parent: Some(&super::KINGS_GAMBIT_ACCEPTED),
    variations: &[
        &ANDERSSEN_DEFENSE,
        &ANDERSSEN_CORDEL_GAMBIT,
        &BERLIN_DEFENSE,
        &BRENTANO_DEFENSE,
        &COTTER_GAMBIT,
        &KOLISCH_DEFENSE,
        &LONG_WHIP,
        &NEUMANN_DEFENSE,
        &PAULSEN_DEFENSE,
        &PAULSEN_DEFENSE_DEFERRED,
        &RICE_GAMBIT,
        &ROSENTHAL_DEFENSE,
    ],
    lines: &[Line {
        parent: &KIESERITZKY_GAMBIT,
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
                role: Pawn,
                from: Square::H2,
                capture: None,
                to: Square::H4,
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
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(49258124950916864),
                    knight: Bitboard(4755801275222720514),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18423944602206601216),
                    white: Bitboard(71135416255),
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
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
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
pub mod anderssen_defense;
pub use anderssen_defense::ANDERSSEN_DEFENSE;
pub mod anderssen_cordel_gambit;
pub use anderssen_cordel_gambit::ANDERSSEN_CORDEL_GAMBIT;
pub mod berlin_defense;
pub use berlin_defense::BERLIN_DEFENSE;
pub mod brentano_defense;
pub use brentano_defense::BRENTANO_DEFENSE;
pub mod cotter_gambit;
pub use cotter_gambit::COTTER_GAMBIT;
pub mod kolisch_defense;
pub use kolisch_defense::KOLISCH_DEFENSE;
pub mod long_whip;
pub use long_whip::LONG_WHIP;
pub mod neumann_defense;
pub use neumann_defense::NEUMANN_DEFENSE;
pub mod paulsen_defense;
pub use paulsen_defense::PAULSEN_DEFENSE;
pub mod paulsen_defense_deferred;
pub use paulsen_defense_deferred::PAULSEN_DEFENSE_DEFERRED;
pub mod rice_gambit;
pub use rice_gambit::RICE_GAMBIT;
pub mod rosenthal_defense;
pub use rosenthal_defense::ROSENTHAL_DEFENSE;
