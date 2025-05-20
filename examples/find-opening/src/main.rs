//! Gets the latest game of a user and asserts that the opening found by `reco`
//! is the same as the opening in Lichess.

use rpgn::{Pgn, Sans};
use shakmaty::san::San;
use shakmaty::{CastlingMode, Chess, Position};

const USER: &str = "german11"; // this guy is a beast

fn main() {
    let mut latest_games = ureq::get(format!("https://lichess.org/api/games/user/{USER}"))
        .query("max", "1")
        .header(http::header::ACCEPT, "application/x-ndjson")
        .call()
        .unwrap()
        .into_body();

    let mut latest_games_reader = pgn_reader::BufferedReader::new(latest_games.as_reader());

    let pgn: Pgn<Sans<San>> = Pgn::from_reader(&mut latest_games_reader).unwrap().unwrap();

    println!("Game PGN: {}", latest_games.read_to_string().unwrap());

    let mut position = pgn
        .fen
        .map(|fen| fen.unwrap().into_position(CastlingMode::Standard).unwrap())
        .unwrap_or_else(Chess::new);
    let mut moves = Vec::with_capacity(pgn.movetext.0.len());

    for san in pgn.movetext.0 {
        let r#move = san.to_move(&position).expect("sans should be legal");
        position = position.play(&r#move).unwrap();
        moves.push(r#move);
    }

    todo!()
}
