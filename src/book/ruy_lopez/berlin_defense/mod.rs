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
#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// Ruy Lopez: Berlin Defense
pub static BERLIN_DEFENSE: Variation = Variation {
    name: "Berlin Defense",
    parent: Some(&super::RUY_LOPEZ),
    variations: &[
        &MORTIMER_VARIATION,
        &NYHOLM_ATTACK,
        &TARRASCH_TRAP,
        &DURAS_VARIATION,
        &MINCKWITZ_VARIATION,
        &CLOSED_SHOWALTER_VARIATION,
        &RIO_GAMBIT_ACCEPTED,
        &ROSENTHAL_VARIATION,
        &TRIFUNOVIC_VARIATION,
        &CLOSED_BERNSTEIN_VARIATION,
        &IMPROVED_STEINITZ_DEFENSE,
        &ZUKERTORT_VARIATION,
        &RIO_DE_JANEIRO_VARIATION,
        &WINAWER_ATTACK,
        &BEVERWIJK_VARIATION,
        &L_HERMET_VARIATION,
        &MORTIMER_TRAP,
        &ANDERSSEN_VARIATION,
        &FISHING_POLE_VARIATION,
        &KAUFMANN_VARIATION,
        &CLOSED_WOLF_VARIATION,
        &CORDEL_VARIATION,
        &HEDGEHOG_VARIATION,
        &BERLIN_WALL,
        &PILLSBURY_VARIATION,
    ],
    lines: &[
        Line {
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<65>(),
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
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
            ],
            setup: Setup {
                board: Board::from_bitboards(
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
                ),
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
        Line {
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<65>(),
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
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
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
                ),
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
    ],
};
pub mod mortimer_variation;
pub use mortimer_variation::MORTIMER_VARIATION;
pub mod nyholm_attack;
pub use nyholm_attack::NYHOLM_ATTACK;
pub mod tarrasch_trap;
pub use tarrasch_trap::TARRASCH_TRAP;
pub mod duras_variation;
pub use duras_variation::DURAS_VARIATION;
pub mod minckwitz_variation;
pub use minckwitz_variation::MINCKWITZ_VARIATION;
pub mod closed_showalter_variation;
pub use closed_showalter_variation::CLOSED_SHOWALTER_VARIATION;
pub mod rio_gambit_accepted;
pub use rio_gambit_accepted::RIO_GAMBIT_ACCEPTED;
pub mod rosenthal_variation;
pub use rosenthal_variation::ROSENTHAL_VARIATION;
pub mod trifunovic_variation;
pub use trifunovic_variation::TRIFUNOVIC_VARIATION;
pub mod closed_bernstein_variation;
pub use closed_bernstein_variation::CLOSED_BERNSTEIN_VARIATION;
pub mod improved_steinitz_defense;
pub use improved_steinitz_defense::IMPROVED_STEINITZ_DEFENSE;
pub mod zukertort_variation;
pub use zukertort_variation::ZUKERTORT_VARIATION;
pub mod rio_de_janeiro_variation;
pub use rio_de_janeiro_variation::RIO_DE_JANEIRO_VARIATION;
pub mod winawer_attack;
pub use winawer_attack::WINAWER_ATTACK;
pub mod beverwijk_variation;
pub use beverwijk_variation::BEVERWIJK_VARIATION;
pub mod l_hermet_variation;
pub use l_hermet_variation::L_HERMET_VARIATION;
pub mod mortimer_trap;
pub use mortimer_trap::MORTIMER_TRAP;
pub mod anderssen_variation;
pub use anderssen_variation::ANDERSSEN_VARIATION;
pub mod fishing_pole_variation;
pub use fishing_pole_variation::FISHING_POLE_VARIATION;
pub mod kaufmann_variation;
pub use kaufmann_variation::KAUFMANN_VARIATION;
pub mod closed_wolf_variation;
pub use closed_wolf_variation::CLOSED_WOLF_VARIATION;
pub mod cordel_variation;
pub use cordel_variation::CORDEL_VARIATION;
pub mod hedgehog_variation;
pub use hedgehog_variation::HEDGEHOG_VARIATION;
pub mod berlin_wall;
pub use berlin_wall::BERLIN_WALL;
pub mod pillsbury_variation;
pub use pillsbury_variation::PILLSBURY_VARIATION;
