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
# use reco::book::NIMZOWITSCH_DEFENSE;
assert_eq!(NIMZOWITSCH_DEFENSE.original_name(), "Nimzowitsch Defense");
```"#
)]
pub static NIMZOWITSCH_DEFENSE: Variation = Variation {
    name: "Nimzowitsch Defense",
    parent: None,
    variations: &[
        &BREYER_VARIATION,
        &COLORADO_COUNTERGAMBIT,
        &COLORADO_COUNTERGAMBIT_ACCEPTED,
        &DECLINED_VARIATION,
        &EL_COLUMPIO_DEFENSE,
        &FRANCO_NIMZOWITSCH_VARIATION,
        &FRENCH_CONNECTION,
        &HORNUNG_GAMBIT,
        &KENNEDY_VARIATION,
        &MIKENAS_VARIATION,
        &NEO_MONGOLOID_DEFENSE,
        &PIRC_CONNECTION,
        &PSEUDO_SPANISH_VARIATION,
        &SCANDINAVIAN_VARIATION,
        &WHEELER_GAMBIT,
        &WILLIAMS_VARIATION,
        &WOODCHUCK_VARIATION,
    ],
    lines: &[
        Line {
            parent: &NIMZOWITSCH_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<0>()),
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
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119329713920),
                        knight: Bitboard(4611690416473899074),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18302351808703496192),
                        white: Bitboard(268496895),
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
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
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
            parent: &NIMZOWITSCH_DEFENSE,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<0>()),
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
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119463929600),
                        knight: Bitboard(4611690416473899074),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18302351808703496192),
                        white: Bitboard(402712575),
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
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
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
pub mod breyer_variation;
pub use breyer_variation::BREYER_VARIATION;
pub mod colorado_countergambit;
pub use colorado_countergambit::COLORADO_COUNTERGAMBIT;
pub mod colorado_countergambit_accepted;
pub use colorado_countergambit_accepted::COLORADO_COUNTERGAMBIT_ACCEPTED;
pub mod declined_variation;
pub use declined_variation::DECLINED_VARIATION;
pub mod el_columpio_defense;
pub use el_columpio_defense::EL_COLUMPIO_DEFENSE;
pub mod franco_nimzowitsch_variation;
pub use franco_nimzowitsch_variation::FRANCO_NIMZOWITSCH_VARIATION;
pub mod french_connection;
pub use french_connection::FRENCH_CONNECTION;
pub mod hornung_gambit;
pub use hornung_gambit::HORNUNG_GAMBIT;
pub mod kennedy_variation;
pub use kennedy_variation::KENNEDY_VARIATION;
pub mod mikenas_variation;
pub use mikenas_variation::MIKENAS_VARIATION;
pub mod neo_mongoloid_defense;
pub use neo_mongoloid_defense::NEO_MONGOLOID_DEFENSE;
pub mod pirc_connection;
pub use pirc_connection::PIRC_CONNECTION;
pub mod pseudo_spanish_variation;
pub use pseudo_spanish_variation::PSEUDO_SPANISH_VARIATION;
pub mod scandinavian_variation;
pub use scandinavian_variation::SCANDINAVIAN_VARIATION;
pub mod wheeler_gambit;
pub use wheeler_gambit::WHEELER_GAMBIT;
pub mod williams_variation;
pub use williams_variation::WILLIAMS_VARIATION;
pub mod woodchuck_variation;
pub use woodchuck_variation::WOODCHUCK_VARIATION;
