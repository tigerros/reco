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
#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// French Defense: Tarrasch Variation
pub static TARRASCH_VARIATION: Variation = Variation {
    name: "Tarrasch Variation",
    parent: Some(&super::FRENCH_DEFENSE),
    variations: &[
        &ELISKASES_VARIATION,
        &HABERDITZ_VARIATION,
        &BOTVINNIK_VARIATION,
        &CLOSED_VARIATION,
        &MODERN_SYSTEM,
        &PAWN_CENTER_VARIATION,
        &GUIMARD_DEFENSE,
        &OPEN_SYSTEM,
        &LENINGRAD_VARIATION,
        &MOROZEVICH_VARIATION,
        &CHISTYAKOV_DEFENSE,
    ],
    lines: &[Line {
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<3>(),
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
                to: E6,
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
                from: D7,
                capture: None,
                to: D5,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: B1,
                capture: None,
                to: D2,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65038346568656640),
                    knight: Bitboard(4755801206503245888),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18439724825837568000),
                    white: Bitboard(402714621),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
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
pub mod eliskases_variation;
pub use eliskases_variation::ELISKASES_VARIATION;
pub mod haberditz_variation;
pub use haberditz_variation::HABERDITZ_VARIATION;
pub mod botvinnik_variation;
pub use botvinnik_variation::BOTVINNIK_VARIATION;
pub mod closed_variation;
pub use closed_variation::CLOSED_VARIATION;
pub mod modern_system;
pub use modern_system::MODERN_SYSTEM;
pub mod pawn_center_variation;
pub use pawn_center_variation::PAWN_CENTER_VARIATION;
pub mod guimard_defense;
pub use guimard_defense::GUIMARD_DEFENSE;
pub mod open_system;
pub use open_system::OPEN_SYSTEM;
pub mod leningrad_variation;
pub use leningrad_variation::LENINGRAD_VARIATION;
pub mod morozevich_variation;
pub use morozevich_variation::MOROZEVICH_VARIATION;
pub mod chistyakov_defense;
pub use chistyakov_defense::CHISTYAKOV_DEFENSE;
