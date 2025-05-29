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
# use reco::book::KADAS_OPENING;
assert_eq!(KADAS_OPENING.original_name(), "Kádas Opening");
```"#
)]
pub static KADAS_OPENING: Variation = Variation {
    name: "Kádas Opening",
    parent: None,
    variations: &[
        &MYERS_VARIATION,
        &STEINBOK_GAMBIT,
        &KOOLA_KOOLA_VARIATION,
        &SCHNEIDER_GAMBIT,
        &KADAS_GAMBIT,
        &BEGINNERS_TRAP,
    ],
    lines: &[Line {
        parent: &KADAS_OPENING,
        code: Code {
            volume: Volume::A,
            category: Category::new_static::<0>(),
        },
        moves: &[Normal {
            role: Pawn,
            from: H2,
            capture: None,
            to: H4,
            promotion: None,
        }],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(71776121208733440),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18446462598732840960),
                    white: Bitboard(2147516415),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(1) {
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
    }],
};
pub mod myers_variation;
pub use myers_variation::MYERS_VARIATION;
pub mod steinbok_gambit;
pub use steinbok_gambit::STEINBOK_GAMBIT;
pub mod koola_koola_variation;
pub use koola_koola_variation::KOOLA_KOOLA_VARIATION;
pub mod schneider_gambit;
pub use schneider_gambit::SCHNEIDER_GAMBIT;
pub mod kadas_gambit;
pub use kadas_gambit::KADAS_GAMBIT;
pub mod beginners_trap;
pub use beginners_trap::BEGINNERS_TRAP;
