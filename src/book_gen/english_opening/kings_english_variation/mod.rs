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
# use reco::book::english_opening::KINGS_ENGLISH_VARIATION;
assert_eq!(KINGS_ENGLISH_VARIATION.original_name(), "English Opening: King's English Variation");
```"#
)]
pub static KINGS_ENGLISH_VARIATION: Variation = Variation {
    name: "King's English Variation",
    parent: Some(&super::ENGLISH_OPENING),
    variations: &[
        &ADHIBAN_GAMBIT,
        &BELLON_GAMBIT,
        &BOTVINNIK_SYSTEM,
        &BREMEN_HORT_VARIATION,
        &CLOSED_SYSTEM,
        &FOUR_KNIGHTS_VARIATION,
        &HUNGARIAN_ATTACK,
        &KAHIKO_HULA_GAMBIT,
        &KERES_DEFENSE,
        &KRAMNIK_SHIROV_COUNTERATTACK,
        &NIMZOWITSCH_VARIATION,
        &NIMZOWITSCH_FLOHR_VARIATION,
        &REVERSED_CLOSED_SICILIAN,
        &REVERSED_SICILIAN,
        &SMYSLOV_DEFENSE,
        &TAIMANOV_VARIATION,
        &THREE_KNIGHTS_SYSTEM,
        &TROGER_DEFENSE,
        &TWO_KNIGHTS_VARIATION,
    ],
    lines: &[
        Line {
            parent: &KINGS_ENGLISH_VARIATION,
            code: Code {
                volume: Volume::A,
                category: Category(RangedU8::new_static::<2>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(67272588220496640),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18441959067824947200),
                        white: Bitboard(67173375),
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
        Line {
            parent: &KINGS_ENGLISH_VARIATION,
            code: Code {
                volume: Volume::A,
                category: Category(RangedU8::new_static::<2>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
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
                    from: Square::B1,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(65029584499833600),
                        knight: Bitboard(4755801206505603072),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18439716064104284160),
                        white: Bitboard(69532605),
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
        },
    ],
};
pub mod adhiban_gambit;
pub use adhiban_gambit::ADHIBAN_GAMBIT;
pub mod bellon_gambit;
pub use bellon_gambit::BELLON_GAMBIT;
pub mod botvinnik_system;
pub use botvinnik_system::BOTVINNIK_SYSTEM;
pub mod bremen_hort_variation;
pub use bremen_hort_variation::BREMEN_HORT_VARIATION;
pub mod closed_system;
pub use closed_system::CLOSED_SYSTEM;
pub mod four_knights_variation;
pub use four_knights_variation::FOUR_KNIGHTS_VARIATION;
pub mod hungarian_attack;
pub use hungarian_attack::HUNGARIAN_ATTACK;
pub mod kahiko_hula_gambit;
pub use kahiko_hula_gambit::KAHIKO_HULA_GAMBIT;
pub mod keres_defense;
pub use keres_defense::KERES_DEFENSE;
pub mod kramnik_shirov_counterattack;
pub use kramnik_shirov_counterattack::KRAMNIK_SHIROV_COUNTERATTACK;
pub mod nimzowitsch_variation;
pub use nimzowitsch_variation::NIMZOWITSCH_VARIATION;
pub mod nimzowitsch_flohr_variation;
pub use nimzowitsch_flohr_variation::NIMZOWITSCH_FLOHR_VARIATION;
pub mod reversed_closed_sicilian;
pub use reversed_closed_sicilian::REVERSED_CLOSED_SICILIAN;
pub mod reversed_sicilian;
pub use reversed_sicilian::REVERSED_SICILIAN;
pub mod smyslov_defense;
pub use smyslov_defense::SMYSLOV_DEFENSE;
pub mod taimanov_variation;
pub use taimanov_variation::TAIMANOV_VARIATION;
pub mod three_knights_system;
pub use three_knights_system::THREE_KNIGHTS_SYSTEM;
pub mod troger_defense;
pub use troger_defense::TROGER_DEFENSE;
pub mod two_knights_variation;
pub use two_knights_variation::TWO_KNIGHTS_VARIATION;
