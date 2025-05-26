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
/// Ruy Lopez
pub static RUY_LOPEZ: Variation = Variation {
    name: "Ruy Lopez",
    parent: None,
    variations: &[
        &HALLOWEEN_ATTACK,
        &MARSHALL_ATTACK,
        &WORMALD_ATTACK,
        &STEINITZ_DEFENSE,
        &POLLOCK_DEFENSE,
        &BIRDS_DEFENSE_DEFERRED,
        &NURNBERG_VARIATION,
        &OLD_STEINITZ_DEFENSE,
        &NOAHS_ARK_TRAP,
        &CLOSED_BERLIN_DEFENSE,
        &CLASSICAL_DEFENSE,
        &CLOSED,
        &BRIX_VARIATION,
        &OPEN,
        &ROTARY_ALBANY_GAMBIT,
        &EXCHANGE_VARIATION,
        &LUCENA_VARIATION,
        &COZIO_DEFENSE,
        &ALAPIN_DEFENSE,
        &EXCHANGE,
        &RETREAT_VARIATION,
        &SPANISH_COUNTERGAMBIT,
        &SCHLIEMANN_DEFENSE,
        &BIRD_VARIATION,
        &BULGARIAN_VARIATION,
        &VINOGRADOV_VARIATION,
        &CENTRAL_COUNTERGAMBIT,
        &RABINOVICH_VARIATION,
        &FIANCHETTO_DEFENSE,
        &BERLIN_DEFENSE,
        &BRENTANO_GAMBIT,
        &MORPHY_DEFENSE,
        &OPEN_BERLIN_DEFENSE,
        &CLASSICAL_VARIATION,
        &STEINITZ_DEFENSE_DEFERRED,
    ],
    lines: &[Line {
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<60>(),
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
                to: B5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272588421820160),
                    knight: Bitboard(4611690416475996162),
                    bishop: Bitboard(2594073393955340292),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18297848277795602432),
                    white: Bitboard(8860528543),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 3,
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
pub mod halloween_attack;
pub use halloween_attack::HALLOWEEN_ATTACK;
pub mod marshall_attack;
pub use marshall_attack::MARSHALL_ATTACK;
pub mod wormald_attack;
pub use wormald_attack::WORMALD_ATTACK;
pub mod steinitz_defense;
pub use steinitz_defense::STEINITZ_DEFENSE;
pub mod pollock_defense;
pub use pollock_defense::POLLOCK_DEFENSE;
pub mod birds_defense_deferred;
pub use birds_defense_deferred::BIRDS_DEFENSE_DEFERRED;
pub mod nurnberg_variation;
pub use nurnberg_variation::NURNBERG_VARIATION;
pub mod old_steinitz_defense;
pub use old_steinitz_defense::OLD_STEINITZ_DEFENSE;
pub mod noahs_ark_trap;
pub use noahs_ark_trap::NOAHS_ARK_TRAP;
pub mod closed_berlin_defense;
pub use closed_berlin_defense::CLOSED_BERLIN_DEFENSE;
pub mod classical_defense;
pub use classical_defense::CLASSICAL_DEFENSE;
pub mod closed;
pub use closed::CLOSED;
pub mod brix_variation;
pub use brix_variation::BRIX_VARIATION;
pub mod open;
pub use open::OPEN;
pub mod rotary_albany_gambit;
pub use rotary_albany_gambit::ROTARY_ALBANY_GAMBIT;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod lucena_variation;
pub use lucena_variation::LUCENA_VARIATION;
pub mod cozio_defense;
pub use cozio_defense::COZIO_DEFENSE;
pub mod alapin_defense;
pub use alapin_defense::ALAPIN_DEFENSE;
pub mod exchange;
pub use exchange::EXCHANGE;
pub mod retreat_variation;
pub use retreat_variation::RETREAT_VARIATION;
pub mod spanish_countergambit;
pub use spanish_countergambit::SPANISH_COUNTERGAMBIT;
pub mod schliemann_defense;
pub use schliemann_defense::SCHLIEMANN_DEFENSE;
pub mod bird_variation;
pub use bird_variation::BIRD_VARIATION;
pub mod bulgarian_variation;
pub use bulgarian_variation::BULGARIAN_VARIATION;
pub mod vinogradov_variation;
pub use vinogradov_variation::VINOGRADOV_VARIATION;
pub mod central_countergambit;
pub use central_countergambit::CENTRAL_COUNTERGAMBIT;
pub mod rabinovich_variation;
pub use rabinovich_variation::RABINOVICH_VARIATION;
pub mod fianchetto_defense;
pub use fianchetto_defense::FIANCHETTO_DEFENSE;
pub mod berlin_defense;
pub use berlin_defense::BERLIN_DEFENSE;
pub mod brentano_gambit;
pub use brentano_gambit::BRENTANO_GAMBIT;
pub mod morphy_defense;
pub use morphy_defense::MORPHY_DEFENSE;
pub mod open_berlin_defense;
pub use open_berlin_defense::OPEN_BERLIN_DEFENSE;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod steinitz_defense_deferred;
pub use steinitz_defense_deferred::STEINITZ_DEFENSE_DEFERRED;
