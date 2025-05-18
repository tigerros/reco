#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByRole, ByColor, Setup};
use core::num::NonZeroU32;
use crate::{Opening, Code, Volume};
use alloc::borrow::Cow;
use deranged::RangedU8;
use core::panic;

#[allow(clippy::doc_markdown, reason = "clippy confuses opening names for items")]/// Italian Game: Giuoco Piano.
pub static GIUOCO_PIANO: [Opening<&str>; 1] = [Opening {
    code: Code {
        volume: Volume::C,
        category: RangedU8::new_static::<50>(),
    },
    name: Cow::Borrowed(&["Italian Game", "Giuoco Piano"]),
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
        role: Bishop,
        from: F8,
        capture: None,
        to: C5,
        promotion: None,
    },
]),
    setup: Cow::Owned(Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(67272588421820160),
                knight: Bitboard(4611690416475996162),
                bishop: Bitboard(288230393398689796),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(15992005285761777664),
                white: Bitboard(337702815)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 4,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) { fullmoves } else { panic!("fullmoves is zero") },
    }),
}];pub mod aitken_variation;
pub use aitken_variation::AITKEN_VARIATION;
pub mod bernstein_variation;
pub use bernstein_variation::BERNSTEIN_VARIATION;
pub mod cracow_variation;
pub use cracow_variation::CRACOW_VARIATION;
pub mod grecos_attack;
pub use grecos_attack::GRECOS_ATTACK;
pub mod holzhausen_attack;
pub use holzhausen_attack::HOLZHAUSEN_ATTACK;
pub mod krause_variation;
pub use krause_variation::KRAUSE_VARIATION;
pub mod rosentreter_variation;
pub use rosentreter_variation::ROSENTRETER_VARIATION;
pub mod steinitz_variation;
pub use steinitz_variation::STEINITZ_VARIATION;
pub mod therkatz_herzog_variation;
pub use therkatz_herzog_variation::THERKATZ_HERZOG_VARIATION;
