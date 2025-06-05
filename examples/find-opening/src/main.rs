//! Gets the latest game of a user and asserts that the opening found by `reco`
//! is the same as the opening in Lichess.

use rpgn::{Pgn, Sans};
use shakmaty::san::San;
use shakmaty::{Chess, Position};

const USER: &str = "tigerros_gh";

fn main() {
    let mut latest_games = ureq::get(format!("https://lichess.org/api/games/user/{USER}"))
        .query("max", "1")
        .query("opening", "true")
        .header(http::header::ACCEPT, "application/x-chess-pgn")
        .call()
        .unwrap()
        .into_body();

    let mut latest_games_reader = pgn_reader::BufferedReader::new(latest_games.as_reader());

    let pgn: Pgn<Sans<San>> = Pgn::from_reader(&mut latest_games_reader).unwrap().unwrap();

    let mut position = Chess::new();

    let mut moves = Vec::with_capacity(pgn.movetext.0.len());

    for san in pgn.movetext.0 {
        let r#move = san.to_move(&position).expect("sans should be legal");
        position = position.play(r#move).unwrap();
        moves.push(r#move);
    }

    let reco_opening = reco::book::find_line_from_moves(Chess::new(), &moves)
        .expect("moves should be legal")
        .expect("opening should be present");

    println!(
        "{:?}",
        reco_opening
            .moves()
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let reco_opening = reco_opening.parent().original_name();

    assert_eq!(
        pgn.other_headers[b"Opening".as_slice()].decode_utf8_lossy(),
        reco_opening
    );
    println!("Opening finder correct");
}
