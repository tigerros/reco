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
use core::panic;pub static FRENCH_DEFENSE: Variation = Variation {
        name: French Defense,
        parent: None,
        variations: &[&burn_variation,
&schlechter_variation,
&chigorin_variation,
&wing_gambit,
&morphy_gambit,
&exchange_variation,
&la_bourdonnais_variation,
&steinitz_attack,
&queen_s_knight,
&marshall_gambit,
&steiner_variation,
&mac_cutcheon_variation,
&banzai_leong_gambit,
&reversed_philidor_formation,
&steinitz_variation,
&carlson_gambit,
&alekhine_chatard_attack,
&franco_hiva_gambit_accepted,
&hoffmann_gambit,
&reti_spielmann_attack,
&classical_variation,
&franco_sicilian_defense,
&orthoschnapp_gambit,
&king_s_indian_attack,
&diemer_duhm_gambit,
&normal_variation,
&paulsen_variation,
&hecht_reefschlager_variation,
&rubinstein_variation,
&two_knights_variation,
&alapin_gambit,
&bird_invitation,
&guimard_variation,
&baeuerle_gambit,
&franco_hiva_gambit,
&pelikan_variation,
&knight_variation,
&henneberger_variation,
&winawer_variation,
&advance_variation,
&perseus_gambit,
&horwitz_attack,
&mediterranean_defense,
&tarrasch_variation,
&st_george_defense,
&diemer_duhm_gambit_accepted],
        lines: &[Line {
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<0>()
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
        to: E6,
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
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(65038346568656640),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18439724825837568000),
                        white: Bitboard(402712575)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) { fullmoves } else { unreachable!() }
            }
        },
Line {
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<0>()
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
        to: E6,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(67290111888387840),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18441976591291514880),
                        white: Bitboard(268496895)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) { fullmoves } else { unreachable!() }
            }
        }]
    }