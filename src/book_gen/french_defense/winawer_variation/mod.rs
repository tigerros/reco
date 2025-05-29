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
# use reco::book::french_defense::WINAWER_VARIATION;
assert_eq!(WINAWER_VARIATION.original_name(), "French Defense: Winawer Variation");
```"#
)]
pub static WINAWER_VARIATION: Variation = Variation {
    name: "Winawer Variation",
    parent: Some(&super::FRENCH_DEFENSE),
    variations: &[
        &CLASSICAL_VARIATION,
        &BOGOLJUBOW_VARIATION,
        &ALEKHINE_MAROCZY_GAMBIT,
        &KONDRATIYEV_VARIATION,
        &POISONED_PAWN_VARIATION,
        &POSITIONAL_VARIATION,
        &WINCKELMANN_REIMER_GAMBIT,
        &DELAYED_EXCHANGE_VARIATION,
        &ADVANCE_VARIATION,
        &MAROCZY_WALLIS_VARIATION,
        &FINGERSLIP_VARIATION,
        &PORTISCH_HOOK_VARIATION,
        &EXCHANGE_VARIATION,
        &ALEKHINE_GAMBIT,
        &ALEKHINE_GAMBIT_ACCEPTED,
        &RETREAT_VARIATION,
        &EINGORN_VARIATION,
        &WARSAW_VARIATION,
        &PETROSIAN_VARIATION,
    ],
    lines: &[Line {
        parent: &WINAWER_VARIATION,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<15>(),
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
                to: C3,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: B4,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(65038346568656640),
                    knight: Bitboard(4755801206503505984),
                    bishop: Bitboard(288230376185266212),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(16133881816657428480),
                    white: Bitboard(402974717),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 2,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
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
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod alekhine_maroczy_gambit;
pub use alekhine_maroczy_gambit::ALEKHINE_MAROCZY_GAMBIT;
pub mod kondratiyev_variation;
pub use kondratiyev_variation::KONDRATIYEV_VARIATION;
pub mod poisoned_pawn_variation;
pub use poisoned_pawn_variation::POISONED_PAWN_VARIATION;
pub mod positional_variation;
pub use positional_variation::POSITIONAL_VARIATION;
pub mod winckelmann_reimer_gambit;
pub use winckelmann_reimer_gambit::WINCKELMANN_REIMER_GAMBIT;
pub mod delayed_exchange_variation;
pub use delayed_exchange_variation::DELAYED_EXCHANGE_VARIATION;
pub mod advance_variation;
pub use advance_variation::ADVANCE_VARIATION;
pub mod maroczy_wallis_variation;
pub use maroczy_wallis_variation::MAROCZY_WALLIS_VARIATION;
pub mod fingerslip_variation;
pub use fingerslip_variation::FINGERSLIP_VARIATION;
pub mod portisch_hook_variation;
pub use portisch_hook_variation::PORTISCH_HOOK_VARIATION;
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod alekhine_gambit;
pub use alekhine_gambit::ALEKHINE_GAMBIT;
pub mod alekhine_gambit_accepted;
pub use alekhine_gambit_accepted::ALEKHINE_GAMBIT_ACCEPTED;
pub mod retreat_variation;
pub use retreat_variation::RETREAT_VARIATION;
pub mod eingorn_variation;
pub use eingorn_variation::EINGORN_VARIATION;
pub mod warsaw_variation;
pub use warsaw_variation::WARSAW_VARIATION;
pub mod petrosian_variation;
pub use petrosian_variation::PETROSIAN_VARIATION;
