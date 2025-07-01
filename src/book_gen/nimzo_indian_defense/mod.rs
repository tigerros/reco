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
# use reco::book::NIMZO_INDIAN_DEFENSE;
assert_eq!(NIMZO_INDIAN_DEFENSE.original_name(), "Nimzo-Indian Defense");
```"#
)]
pub static NIMZO_INDIAN_DEFENSE: Variation = Variation {
    name: "Nimzo-Indian Defense",
    parent: None,
    variations: &[
        &CLASSICAL_VARIATION,
        &DILWORTH_GAMBIT,
        &KMOCH_VARIATION,
        &LENINGRAD_VARIATION,
        &MIKENAS_ATTACK,
        &NORMAL_VARIATION,
        &RAGOZIN_DEFENSE,
        &RAGOZIN_VARIATION,
        &RESHEVSKY_VARIATION,
        &ROMANISHIN_VARIATION,
        &RUBINSTEIN_SYSTEM,
        &SIMAGIN_VARIATION,
        &SPIELMANN_VARIATION,
        &ST_PETERSBURG_VARIATION,
        &SAMISCH_VARIATION,
        &THREE_KNIGHTS_VARIATION,
    ],
    lines: &[
        Line {
            parent: &NIMZO_INDIAN_DEFENSE,
            code: Code {
                volume: Volume::E,
                category: Category(RangedU8::new_static::<2>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
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
                    to: Square::E6,
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
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(67290111821280000),
                        knight: Bitboard(144150372448206912),
                        bishop: Bitboard(288230376185266212),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(11524482748056076288),
                        white: Bitboard(201651197),
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
        },
        Line {
            parent: &NIMZO_INDIAN_DEFENSE,
            code: Code {
                volume: Volume::E,
                category: Category(RangedU8::new_static::<5>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
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
                    to: Square::E6,
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
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E3,
                    promotion: None,
                },
                Castle {
                    king: Square::E8,
                    rook: Square::H8,
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
                        pawn: Bitboard(67290111822324480),
                        knight: Bitboard(144150372450304000),
                        bishop: Bitboard(288230376185266212),
                        rook: Bitboard(2377900603251622017),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(4611686018427387920),
                    },
                    ByColor {
                        black: Bitboard(8065718234235535360),
                        white: Bitboard(204792765),
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
                castling_rights: Bitboard(129),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 2,
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
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod dilworth_gambit;
pub use dilworth_gambit::DILWORTH_GAMBIT;
pub mod kmoch_variation;
pub use kmoch_variation::KMOCH_VARIATION;
pub mod leningrad_variation;
pub use leningrad_variation::LENINGRAD_VARIATION;
pub mod mikenas_attack;
pub use mikenas_attack::MIKENAS_ATTACK;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod ragozin_defense;
pub use ragozin_defense::RAGOZIN_DEFENSE;
pub mod ragozin_variation;
pub use ragozin_variation::RAGOZIN_VARIATION;
pub mod reshevsky_variation;
pub use reshevsky_variation::RESHEVSKY_VARIATION;
pub mod romanishin_variation;
pub use romanishin_variation::ROMANISHIN_VARIATION;
pub mod rubinstein_system;
pub use rubinstein_system::RUBINSTEIN_SYSTEM;
pub mod simagin_variation;
pub use simagin_variation::SIMAGIN_VARIATION;
pub mod spielmann_variation;
pub use spielmann_variation::SPIELMANN_VARIATION;
pub mod st_petersburg_variation;
pub use st_petersburg_variation::ST_PETERSBURG_VARIATION;
pub mod samisch_variation;
pub use samisch_variation::SAMISCH_VARIATION;
pub mod three_knights_variation;
pub use three_knights_variation::THREE_KNIGHTS_VARIATION;
