#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByRole, ByColor, Setup};
use core::num::NonZeroU32;
use crate::{Variation, Line, Code, Volume, Category};
use core::panic;pub static RUY_LOPEZ: Variation = Variation {
        name: Ruy Lopez,
        parent: None,
        variations: &[&rabinovich_variation,
&cozio_defense,
&old_steinitz_defense,
&central_countergambit,
&rotary_albany_gambit,
&exchange_variation,
&steinitz_defense_deferred,
&noah_s_ark_trap,
&fianchetto_defense,
&berlin_defense,
&wormald_attack,
&pollock_defense,
&spanish_countergambit,
&open_berlin_defense,
&alapin_defense,
&brentano_gambit,
&closed_berlin_defense,
&marshall_attack,
&bulgarian_variation,
&open,
&bird_s_defense_deferred,
&vinogradov_variation,
&schliemann_defense,
&bird_variation,
&morphy_defense,
&classical_variation,
&halloween_attack,
&retreat_variation,
&exchange,
&brix_variation,
&lucena_variation,
&classical_defense,
&closed,
&steinitz_defense,
&nurnberg_variation],
        lines: &[Line {
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<60>()
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
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(67272588421820160),
                        knight: Bitboard(4611690416475996162),
                        bishop: Bitboard(2594073393955340292),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18297848277795602432),
                        white: Bitboard(8860528543)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 3,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) { fullmoves } else { unreachable!() }
            }
        }]
    }