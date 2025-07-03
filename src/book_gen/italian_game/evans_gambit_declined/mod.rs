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
# use reco::book::italian_game::EVANS_GAMBIT_DECLINED;
assert_eq!(EVANS_GAMBIT_DECLINED.original_name(), "Italian Game: Evans Gambit Declined");
```"#
)]
pub static EVANS_GAMBIT_DECLINED: Variation = Variation {
    name: "Evans Gambit Declined",
    parent: Some(&super::ITALIAN_GAME),
    variations: &[
        &CORDEL_VARIATION,
        &HICKEN_VARIATION,
        &HIRSCHBACH_VARIATION,
        &LANGE_VARIATION,
        &PAVLOV_VARIATION,
        &SHOWALTER_VARIATION,
        &VASQUEZ_VARIATION,
    ],
    lines: &[
        Line {
            parent: &EVANS_GAMBIT_DECLINED,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<5>()),
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
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B2,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C5,
                    capture: None,
                    to: Square::B6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(67272588455374080),
                        knight: Bitboard(4611690416475996162),
                        bishop: Bitboard(288232575242076164),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(15992007467605164032),
                        white: Bitboard(371256735),
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
        },
        Line {
            parent: &EVANS_GAMBIT_DECLINED,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<5>()),
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
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B2,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C5,
                    capture: None,
                    to: Square::B6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A2,
                    capture: None,
                    to: Square::A4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(67272588472151040),
                        knight: Bitboard(4611690416475996162),
                        bishop: Bitboard(288232575242076164),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(15992007467605164032),
                        white: Bitboard(388033695),
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
                halfmoves: 0,
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
        },
    ],
};
pub mod cordel_variation;
pub use cordel_variation::CORDEL_VARIATION;
pub mod hicken_variation;
pub use hicken_variation::HICKEN_VARIATION;
pub mod hirschbach_variation;
pub use hirschbach_variation::HIRSCHBACH_VARIATION;
pub mod lange_variation;
pub use lange_variation::LANGE_VARIATION;
pub mod pavlov_variation;
pub use pavlov_variation::PAVLOV_VARIATION;
pub mod showalter_variation;
pub use showalter_variation::SHOWALTER_VARIATION;
pub mod vasquez_variation;
pub use vasquez_variation::VASQUEZ_VARIATION;
