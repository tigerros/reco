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
# use reco::book::caro_kann_defense::CLASSICAL_VARIATION;
assert_eq!(CLASSICAL_VARIATION.original_name(), "Caro-Kann Defense: Classical Variation");
```"#
)]
pub static CLASSICAL_VARIATION: Variation = Variation {
    name: "Classical Variation",
    parent: Some(&super::CARO_KANN_DEFENSE),
    variations: &[
        &SEIRAWAN_VARIATION,
        &FLOHR_VARIATION,
        &SPASSKY_VARIATION,
        &MAIN_LINE,
        &LOBRON_SYSTEM,
        &MAROCZY_ATTACK,
    ],
    lines: &[
        Line {
            parent: &CLASSICAL_VARIATION,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<19>(),
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
                    from: C7,
                    capture: None,
                    to: C6,
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
                    to: D2,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D5,
                    capture: Some(Pawn),
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: D2,
                    capture: Some(Pawn),
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C8,
                    capture: None,
                    to: F5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: E4,
                    capture: None,
                    to: G3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: F5,
                    capture: None,
                    to: G6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: H2,
                    capture: None,
                    to: H4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: H7,
                    capture: None,
                    to: H6,
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
                    to: D7,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(32514760138319616),
                        knight: Bitboard(4613937818247364608),
                        bishop: Bitboard(2305913377957871652),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(17977177841858510848),
                        white: Bitboard(2288019389),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 2,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
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
            parent: &CLASSICAL_VARIATION,
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<18>(),
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
                    from: C7,
                    capture: None,
                    to: C6,
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
                    to: D2,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D5,
                    capture: Some(Pawn),
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: D2,
                    capture: Some(Pawn),
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C8,
                    capture: None,
                    to: F5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(68402817521477376),
                        knight: Bitboard(4755801206771679296),
                        bishop: Bitboard(2305843146652647460),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18154859058346065920),
                        white: Bitboard(402712573),
                    },
                ),
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
    ],
};
pub mod seirawan_variation;
pub use seirawan_variation::SEIRAWAN_VARIATION;
pub mod flohr_variation;
pub use flohr_variation::FLOHR_VARIATION;
pub mod spassky_variation;
pub use spassky_variation::SPASSKY_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod lobron_system;
pub use lobron_system::LOBRON_SYSTEM;
pub mod maroczy_attack;
pub use maroczy_attack::MAROCZY_ATTACK;
