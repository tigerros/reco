use reco::Opening;

/// Returns a string of a const expression which represents the given opening.
///
/// Uses the last variation layer for the constant identifier, or the opening name if the
/// variation is unspecified.
pub fn get_opening_constant_expression_string(opening: &Opening<String>) -> String {
    let moves = &opening.moves;
    let volume = opening.code.volume;
    let category = opening.code.category;
    let name_joined = opening
        .name
        .iter()
        .map(|v| format!("\"{}\"", v.trim()))
        .collect::<Vec<_>>()
        .join(", ");
    let (by_role_bitboard, by_color_bitboard) = opening.setup.board.clone().into_bitboards();
    let promoted = opening.setup.promoted.0;
    let pockets = opening.setup.pockets;
    let turn = opening.setup.turn;
    let castling_rights = opening.setup.castling_rights.0;
    let ep_square = opening.setup.ep_square;
    let remaining_checks = opening.setup.remaining_checks;
    let halfmoves = opening.setup.halfmoves;
    let fullmoves = opening.setup.fullmoves;

    format!(
        r#"Opening {{
    code: Code {{
        volume: Volume::{volume},
        category: RangedU8::new_static::<{category}>(),
    }},
    name: Cow::Borrowed(&[{name_joined}]),
    moves: Cow::Borrowed(&{moves:#?}),
    setup: Cow::Owned(Setup {{
        board: Board::from_bitboards(
            ByRole {{
                pawn: Bitboard({}),
                knight: Bitboard({}),
                bishop: Bitboard({}),
                rook: Bitboard({}),
                queen: Bitboard({}),
                king: Bitboard({})
            }},
            ByColor {{
                black: Bitboard({}),
                white: Bitboard({})
            }}
        ),
        promoted: Bitboard({promoted}),
        pockets: {pockets:#?},
        turn: {turn:#?},
        castling_rights: Bitboard({castling_rights}),
        ep_square: {ep_square:#?},
        remaining_checks: {remaining_checks:#?},
        halfmoves: {halfmoves},
        fullmoves: if let Some(fullmoves) = NonZeroU32::new({fullmoves}) {{ fullmoves }} else {{ panic!("fullmoves is zero") }},
    }}),
}}"#,
        by_role_bitboard.pawn.0,
        by_role_bitboard.knight.0,
        by_role_bitboard.bishop.0,
        by_role_bitboard.rook.0,
        by_role_bitboard.queen.0,
        by_role_bitboard.king.0,
        by_color_bitboard.black.0,
        by_color_bitboard.white.0
    )
}
