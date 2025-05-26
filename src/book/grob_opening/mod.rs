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
/// Grob Opening
pub static GROB_OPENING: Variation = Variation {
    name: "Grob Opening",
    parent: None,
    variations: &[
        &GROB_GAMBIT,
        &ROMFORD_COUNTERGAMBIT,
        &SPIKE,
        &LONDON_DEFENSE,
        &KEENE_DEFENSE,
        &GROB_GAMBIT_DECLINED,
        &DOUBLE_GROB,
        &SPIKE_ATTACK,
        &ZILBERMINTS_GAMBIT,
        &ALESSI_GAMBIT,
    ],
    lines: &[Line {
        code: Code {
            volume: Volume::A,
            category: Category::new_static::<0>(),
        },
        moves: &[Normal {
            role: Pawn,
            from: G2,
            capture: None,
            to: G4,
            promotion: None,
        }],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(71776120135008000),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18446462598732840960),
                    white: Bitboard(1073790975),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(1) {
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
pub mod grob_gambit;
pub use grob_gambit::GROB_GAMBIT;
pub mod romford_countergambit;
pub use romford_countergambit::ROMFORD_COUNTERGAMBIT;
pub mod spike;
pub use spike::SPIKE;
pub mod london_defense;
pub use london_defense::LONDON_DEFENSE;
pub mod keene_defense;
pub use keene_defense::KEENE_DEFENSE;
pub mod grob_gambit_declined;
pub use grob_gambit_declined::GROB_GAMBIT_DECLINED;
pub mod double_grob;
pub use double_grob::DOUBLE_GROB;
pub mod spike_attack;
pub use spike_attack::SPIKE_ATTACK;
pub mod zilbermints_gambit;
pub use zilbermints_gambit::ZILBERMINTS_GAMBIT;
pub mod alessi_gambit;
pub use alessi_gambit::ALESSI_GAMBIT;
