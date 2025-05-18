use crate::{Code, Opening, Volume};
use alloc::borrow::Cow;
use core::num::NonZeroU32;
use core::panic;
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
    clippy::enum_glob_use,
    reason = "there's 64 variants in this enum, importing them all is stupid"
)]
use shakmaty::Square::*;
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByColor, ByRole, Setup};

#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// French Defense: Winawer Variation.
pub static WINAWER_VARIATION: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<15>(),
    },
    name: Cow::Borrowed(&["French Defense", "Winawer Variation"]),
    moves: Cow::Borrowed(&[
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
    ]),
    setup: Cow::Owned(Setup {
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
            panic!("fullmoves is zero")
        },
    }),
}];
pub mod advance_variation;
pub use advance_variation::ADVANCE_VARIATION;
pub mod alekhine_gambit;
pub mod alekhine_gambit_accepted;
pub use alekhine_gambit_accepted::ALEKHINE_GAMBIT_ACCEPTED;
pub mod alekhine_maroczy_gambit;
pub use alekhine_maroczy_gambit::ALEKHINE_MAROCZY_GAMBIT;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod delayed_exchange_variation;
pub use delayed_exchange_variation::DELAYED_EXCHANGE_VARIATION;
pub mod eingorn_variation;
pub use eingorn_variation::EINGORN_VARIATION;
pub mod exchange_variation;
pub mod fingerslip_variation;
pub use fingerslip_variation::FINGERSLIP_VARIATION;
pub mod kondratiyev_variation;
pub use kondratiyev_variation::KONDRATIYEV_VARIATION;
pub mod maroczy_wallis_variation;
pub use maroczy_wallis_variation::MAROCZY_WALLIS_VARIATION;
pub mod petrosian_variation;
pub use petrosian_variation::PETROSIAN_VARIATION;
pub mod poisoned_pawn_variation;
pub use poisoned_pawn_variation::POISONED_PAWN_VARIATION;
pub mod portisch_hook_variation;
pub use portisch_hook_variation::PORTISCH_HOOK_VARIATION;
pub mod positional_variation;
pub use positional_variation::POSITIONAL_VARIATION;
pub mod retreat_variation;
pub use retreat_variation::RETREAT_VARIATION;
pub mod warsaw_variation;
pub use warsaw_variation::WARSAW_VARIATION;
pub mod winckelmann_reimer_gambit;
pub use winckelmann_reimer_gambit::WINCKELMANN_REIMER_GAMBIT;
