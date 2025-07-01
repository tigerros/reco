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
# use reco::book::ruy_lopez::BERLIN_DEFENSE;
assert_eq!(BERLIN_DEFENSE.original_name(), "Ruy Lopez: Berlin Defense");
```"#
)]
pub static BERLIN_DEFENSE: Variation = Variation {
    name: "Berlin Defense",
    parent: Some(&super::RUY_LOPEZ),
    variations: &[
        &ANDERSSEN_VARIATION,
        &BERLIN_WALL,
        &BEVERWIJK_VARIATION,
        &CLOSED_BERNSTEIN_VARIATION,
        &CLOSED_SHOWALTER_VARIATION,
        &CLOSED_WOLF_VARIATION,
        &CORDEL_VARIATION,
        &DURAS_VARIATION,
        &FISHING_POLE_VARIATION,
        &HEDGEHOG_VARIATION,
        &IMPROVED_STEINITZ_DEFENSE,
        &KAUFMANN_VARIATION,
        &MINCKWITZ_VARIATION,
        &MORTIMER_TRAP,
        &MORTIMER_VARIATION,
        &NYHOLM_ATTACK,
        &PILLSBURY_VARIATION,
        &RIO_GAMBIT_ACCEPTED,
        &RIO_DE_JANEIRO_VARIATION,
        &ROSENTHAL_VARIATION,
        &TARRASCH_TRAP,
        &TRIFUNOVIC_VARIATION,
        &WINAWER_ATTACK,
        &ZUKERTORT_VARIATION,
        &L_HERMET_VARIATION,
    ],
    lines: &[
        Line {
            parent: &BERLIN_DEFENSE,
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
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(67272588421820160),
                        knight: Bitboard(39582420697090),
                        bishop: Bitboard(2594073393955340292),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13686197443740303360),
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
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 4,
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
            parent: &BERLIN_DEFENSE,
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
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(67272588421820160),
                        knight: Bitboard(39582420697090),
                        bishop: Bitboard(2594073393955340292),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(13686197443740303360),
                        white: Bitboard(8860528495),
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
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 5,
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
    ],
};
pub mod anderssen_variation;
pub use anderssen_variation::ANDERSSEN_VARIATION;
pub mod berlin_wall;
pub use berlin_wall::BERLIN_WALL;
pub mod beverwijk_variation;
pub use beverwijk_variation::BEVERWIJK_VARIATION;
pub mod closed_bernstein_variation;
pub use closed_bernstein_variation::CLOSED_BERNSTEIN_VARIATION;
pub mod closed_showalter_variation;
pub use closed_showalter_variation::CLOSED_SHOWALTER_VARIATION;
pub mod closed_wolf_variation;
pub use closed_wolf_variation::CLOSED_WOLF_VARIATION;
pub mod cordel_variation;
pub use cordel_variation::CORDEL_VARIATION;
pub mod duras_variation;
pub use duras_variation::DURAS_VARIATION;
pub mod fishing_pole_variation;
pub use fishing_pole_variation::FISHING_POLE_VARIATION;
pub mod hedgehog_variation;
pub use hedgehog_variation::HEDGEHOG_VARIATION;
pub mod improved_steinitz_defense;
pub use improved_steinitz_defense::IMPROVED_STEINITZ_DEFENSE;
pub mod kaufmann_variation;
pub use kaufmann_variation::KAUFMANN_VARIATION;
pub mod minckwitz_variation;
pub use minckwitz_variation::MINCKWITZ_VARIATION;
pub mod mortimer_trap;
pub use mortimer_trap::MORTIMER_TRAP;
pub mod mortimer_variation;
pub use mortimer_variation::MORTIMER_VARIATION;
pub mod nyholm_attack;
pub use nyholm_attack::NYHOLM_ATTACK;
pub mod pillsbury_variation;
pub use pillsbury_variation::PILLSBURY_VARIATION;
pub mod rio_gambit_accepted;
pub use rio_gambit_accepted::RIO_GAMBIT_ACCEPTED;
pub mod rio_de_janeiro_variation;
pub use rio_de_janeiro_variation::RIO_DE_JANEIRO_VARIATION;
pub mod rosenthal_variation;
pub use rosenthal_variation::ROSENTHAL_VARIATION;
pub mod tarrasch_trap;
pub use tarrasch_trap::TARRASCH_TRAP;
pub mod trifunovic_variation;
pub use trifunovic_variation::TRIFUNOVIC_VARIATION;
pub mod winawer_attack;
pub use winawer_attack::WINAWER_ATTACK;
pub mod zukertort_variation;
pub use zukertort_variation::ZUKERTORT_VARIATION;
pub mod l_hermet_variation;
pub use l_hermet_variation::L_HERMET_VARIATION;
