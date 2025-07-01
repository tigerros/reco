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
# use reco::book::queens_pawn_game::london_system::PTERODACTYL_VARIATION;
assert_eq!(PTERODACTYL_VARIATION.original_name(), "Queen's Pawn Game: London System, Pterodactyl Variation");
```"#
)]
pub static PTERODACTYL_VARIATION: Variation = Variation {
    name: "Pterodactyl Variation",
    parent: Some(&super::LONDON_SYSTEM),
    variations: &[],
    lines: &[Line {
        parent: &PTERODACTYL_VARIATION,
        code: Code {
            volume: Volume::D,
            category: Category(RangedU8::new_static::<0>()),
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
                role: Pawn,
                from: Square::G7,
                capture: None,
                to: Square::G6,
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
                role: Bishop,
                from: Square::F8,
                capture: None,
                to: Square::G7,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::C1,
                capture: None,
                to: Square::F4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C7,
                capture: None,
                to: Square::C5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C2,
                capture: None,
                to: Square::C3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C5,
                capture: Some(Pawn),
                to: Square::D4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C3,
                capture: Some(Pawn),
                to: Square::D4,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: Square::D8,
                capture: None,
                to: Square::A5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(52706189523350272),
                    knight: Bitboard(4755801206505340930),
                    bishop: Bitboard(306244775198064672),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(4294967304),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(15563103310348025856),
                    white: Bitboard(673248187),
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
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
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
