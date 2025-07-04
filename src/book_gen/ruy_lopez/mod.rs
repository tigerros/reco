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
# use reco::book::RUY_LOPEZ;
assert_eq!(RUY_LOPEZ.original_name(), "Ruy Lopez");
```"#
)]
pub static RUY_LOPEZ: Variation = Variation {
    name: "Ruy Lopez",
    parent: None,
    variations: &[
        &ALAPIN_DEFENSE,
        &BERLIN_DEFENSE,
        &BIRD_VARIATION,
        &BIRDS_DEFENSE_DEFERRED,
        &BRENTANO_GAMBIT,
        &BRIX_VARIATION,
        &BULGARIAN_VARIATION,
        &CENTRAL_COUNTERGAMBIT,
        &CLASSICAL_DEFENSE,
        &CLASSICAL_VARIATION,
        &CLOSED,
        &CLOSED_BERLIN_DEFENSE,
        &COZIO_DEFENSE,
        &EXCHANGE,
        &EXCHANGE_VARIATION,
        &FIANCHETTO_DEFENSE,
        &HALLOWEEN_ATTACK,
        &LUCENA_VARIATION,
        &MARSHALL_ATTACK,
        &MORPHY_DEFENSE,
        &NOAHS_ARK_TRAP,
        &NURNBERG_VARIATION,
        &OLD_STEINITZ_DEFENSE,
        &OPEN,
        &OPEN_BERLIN_DEFENSE,
        &POLLOCK_DEFENSE,
        &RABINOVICH_VARIATION,
        &RETREAT_VARIATION,
        &ROTARY_ALBANY_GAMBIT,
        &SCHLIEMANN_DEFENSE,
        &SPANISH_COUNTERGAMBIT,
        &STEINITZ_DEFENSE,
        &STEINITZ_DEFENSE_DEFERRED,
        &VINOGRADOV_VARIATION,
        &WORMALD_ATTACK,
    ],
    lines: &[Line {
        parent: &RUY_LOPEZ,
        code: Code {
            volume: Volume::C,
            category: Category(RangedU8::new_static::<6>()),
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
                role: Knight,
                from: Square::G1,
                capture: None,
                to: Square::F3,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::B8,
                capture: None,
                to: Square::C6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F1,
                capture: None,
                to: Square::B5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
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
pub mod alapin_defense;
pub use alapin_defense::ALAPIN_DEFENSE;
pub mod berlin_defense;
pub use berlin_defense::BERLIN_DEFENSE;
pub mod bird_variation;
pub use bird_variation::BIRD_VARIATION;
pub mod birds_defense_deferred;
pub use birds_defense_deferred::BIRDS_DEFENSE_DEFERRED;
pub mod brentano_gambit;
pub use brentano_gambit::BRENTANO_GAMBIT;
pub mod brix_variation;
pub use brix_variation::BRIX_VARIATION;
pub mod bulgarian_variation;
pub use bulgarian_variation::BULGARIAN_VARIATION;
pub mod central_countergambit;
pub use central_countergambit::CENTRAL_COUNTERGAMBIT;
pub mod classical_defense;
pub use classical_defense::CLASSICAL_DEFENSE;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod closed;
pub use closed::CLOSED;
pub mod closed_berlin_defense;
pub use closed_berlin_defense::CLOSED_BERLIN_DEFENSE;
pub mod cozio_defense;
pub use cozio_defense::COZIO_DEFENSE;
pub mod exchange;
pub use exchange::EXCHANGE;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod fianchetto_defense;
pub use fianchetto_defense::FIANCHETTO_DEFENSE;
pub mod halloween_attack;
pub use halloween_attack::HALLOWEEN_ATTACK;
pub mod lucena_variation;
pub use lucena_variation::LUCENA_VARIATION;
pub mod marshall_attack;
pub use marshall_attack::MARSHALL_ATTACK;
pub mod morphy_defense;
pub use morphy_defense::MORPHY_DEFENSE;
pub mod noahs_ark_trap;
pub use noahs_ark_trap::NOAHS_ARK_TRAP;
pub mod nurnberg_variation;
pub use nurnberg_variation::NURNBERG_VARIATION;
pub mod old_steinitz_defense;
pub use old_steinitz_defense::OLD_STEINITZ_DEFENSE;
pub mod open;
pub use open::OPEN;
pub mod open_berlin_defense;
pub use open_berlin_defense::OPEN_BERLIN_DEFENSE;
pub mod pollock_defense;
pub use pollock_defense::POLLOCK_DEFENSE;
pub mod rabinovich_variation;
pub use rabinovich_variation::RABINOVICH_VARIATION;
pub mod retreat_variation;
pub use retreat_variation::RETREAT_VARIATION;
pub mod rotary_albany_gambit;
pub use rotary_albany_gambit::ROTARY_ALBANY_GAMBIT;
pub mod schliemann_defense;
pub use schliemann_defense::SCHLIEMANN_DEFENSE;
pub mod spanish_countergambit;
pub use spanish_countergambit::SPANISH_COUNTERGAMBIT;
pub mod steinitz_defense;
pub use steinitz_defense::STEINITZ_DEFENSE;
pub mod steinitz_defense_deferred;
pub use steinitz_defense_deferred::STEINITZ_DEFENSE_DEFERRED;
pub mod vinogradov_variation;
pub use vinogradov_variation::VINOGRADOV_VARIATION;
pub mod wormald_attack;
pub use wormald_attack::WORMALD_ATTACK;
