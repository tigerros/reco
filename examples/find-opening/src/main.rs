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

    let mut reco_opening = reco::find_from_moves(&moves, Chess::new())
        .expect("moves should be legal")
        .expect("opening should be present")
        .name
        .to_vec();

    let first = reco_opening.remove(0);
    let mut rest_joined = reco_opening.join(", ");

    if !rest_joined.is_empty() {
        rest_joined.insert_str(0, ": ");
    }

    let reco_opening_string = format!("{first}{rest_joined}");

    assert_eq!(
        pgn.other_headers[b"Opening".as_slice()].decode_utf8_lossy(),
        reco_opening_string
    );
    println!("Opening finder correct");
}
