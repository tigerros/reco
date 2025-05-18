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
/// Ruy Lopez: Closed.
pub static CLOSED: [Opening<&str>; 5] = [
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<84>(),
        },
        name: Cow::Borrowed(&["Ruy Lopez", "Closed"]),
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
                to: B5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: A7,
                capture: None,
                to: A6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: B5,
                capture: None,
                to: A4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(66992212956737280),
                    knight: Bitboard(39582420697090),
                    bishop: Bitboard(292733975795859460),
                    rook: Bitboard(9295429630892703777),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606847040),
                },
                ByColor {
                    black: Bitboard(11384577658688897024),
                    white: Bitboard(287371119),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703744),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 4,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<88>(),
        },
        name: Cow::Borrowed(&["Ruy Lopez", "Closed"]),
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
                to: B5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: A7,
                capture: None,
                to: A6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: B5,
                capture: None,
                to: A4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: F1,
                capture: None,
                to: E1,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: B7,
                capture: None,
                to: B5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: A4,
                capture: None,
                to: B3,
                promotion: None,
            },
            Castle { king: E8, rook: H8 },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(66429271593250560),
                    knight: Bitboard(39582420697090),
                    bishop: Bitboard(292733975779213316),
                    rook: Bitboard(2377900603251621905),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7925250203504869376),
                    white: Bitboard(270724959),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(0),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 2,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<88>(),
        },
        name: Cow::Borrowed(&["Ruy Lopez", "Closed"]),
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
                to: B5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: A7,
                capture: None,
                to: A6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: B5,
                capture: None,
                to: A4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: F1,
                capture: None,
                to: E1,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: B7,
                capture: None,
                to: B5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: A4,
                capture: None,
                to: B3,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(66429271593250560),
                    knight: Bitboard(39582420697090),
                    bishop: Bitboard(292733975779213316),
                    rook: Bitboard(9295429630892703761),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606847040),
                },
                ByColor {
                    black: Bitboard(11384014717325410304),
                    white: Bitboard(270724959),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703744),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<90>(),
        },
        name: Cow::Borrowed(&["Ruy Lopez", "Closed"]),
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
                to: B5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: A7,
                capture: None,
                to: A6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: B5,
                capture: None,
                to: A4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: F1,
                capture: None,
                to: E1,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: B7,
                capture: None,
                to: B5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: A4,
                capture: None,
                to: B3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D7,
                capture: None,
                to: D6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C2,
                capture: None,
                to: C3,
                promotion: None,
            },
            Castle { king: E8, rook: H8 },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(64186267872848640),
                    knight: Bitboard(39582420697090),
                    bishop: Bitboard(292733975779213316),
                    rook: Bitboard(2377900603251621905),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7923007199784206336),
                    white: Bitboard(270986079),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(0),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<92>(),
        },
        name: Cow::Borrowed(&["Ruy Lopez", "Closed"]),
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
                to: B5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: A7,
                capture: None,
                to: A6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: B5,
                capture: None,
                to: A4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: G8,
                capture: None,
                to: F6,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: E7,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: F1,
                capture: None,
                to: E1,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: B7,
                capture: None,
                to: B5,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: A4,
                capture: None,
                to: B3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: D7,
                capture: None,
                to: D6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C2,
                capture: None,
                to: C3,
                promotion: None,
            },
            Castle { king: E8, rook: H8 },
            Normal {
                role: Pawn,
                from: H2,
                capture: None,
                to: H3,
                promotion: None,
            },
        ]),
        setup: Cow::Owned(Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(64186267881204480),
                    knight: Bitboard(39582420697090),
                    bishop: Bitboard(292733975779213316),
                    rook: Bitboard(2377900603251621905),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(7923007199784206336),
                    white: Bitboard(279341919),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(0),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        }),
    },
];
pub mod alekhine_gambit;
pub use alekhine_gambit::ALEKHINE_GAMBIT;
pub mod anti_marshall;
pub use anti_marshall::ANTI_MARSHALL;
pub mod averbakh_variation;
pub use averbakh_variation::AVERBAKH_VARIATION;
pub mod balla_variation;
pub use balla_variation::BALLA_VARIATION;
pub mod basque_gambit;
pub use basque_gambit::BASQUE_GAMBIT;
pub mod bogoljubow_variation;
pub use bogoljubow_variation::BOGOLJUBOW_VARIATION;
pub mod borisenko_variation;
pub use borisenko_variation::BORISENKO_VARIATION;
pub mod breyer;
pub use breyer::BREYER;
pub mod breyer_defense;
pub use breyer_defense::BREYER_DEFENSE;
pub mod center_attack;
pub use center_attack::CENTER_ATTACK;
pub mod chigorin;
pub mod chigorin_defense;
pub use chigorin_defense::CHIGORIN_DEFENSE;
pub mod delayed_exchange;
pub use delayed_exchange::DELAYED_EXCHANGE;
pub mod flohr_system;
pub use flohr_system::FLOHR_SYSTEM;
pub mod karpov_variation;
pub use karpov_variation::KARPOV_VARIATION;
pub mod kecskemet_variation;
pub use kecskemet_variation::KECSKEMET_VARIATION;
pub mod keres_defense;
pub use keres_defense::KERES_DEFENSE;
pub mod kholmov_variation;
pub use kholmov_variation::KHOLMOV_VARIATION;
pub mod leonhardt_variation;
pub use leonhardt_variation::LEONHARDT_VARIATION;
pub mod lutikov_variation;
pub use lutikov_variation::LUTIKOV_VARIATION;
pub mod martinez_variation;
pub use martinez_variation::MARTINEZ_VARIATION;
pub mod morphy_attack;
pub use morphy_attack::MORPHY_ATTACK;
pub mod pilnik_variation;
pub use pilnik_variation::PILNIK_VARIATION;
pub mod rosen_attack;
pub use rosen_attack::ROSEN_ATTACK;
pub mod rossolimo_defense;
pub use rossolimo_defense::ROSSOLIMO_DEFENSE;
pub mod smyslov_defense;
pub use smyslov_defense::SMYSLOV_DEFENSE;
pub mod smyslov_breyer_zaitsev_hybrid;
pub use smyslov_breyer_zaitsev_hybrid::SMYSLOV_BREYER_ZAITSEV_HYBRID;
pub mod suetin_variation;
pub use suetin_variation::SUETIN_VARIATION;
pub mod trajkovic_counterattack;
pub use trajkovic_counterattack::TRAJKOVIC_COUNTERATTACK;
pub mod worrall_attack;
pub use worrall_attack::WORRALL_ATTACK;
pub mod yates_variation;
pub use yates_variation::YATES_VARIATION;
pub mod zaitsev_system;
pub use zaitsev_system::ZAITSEV_SYSTEM;
