//! Gets the latest game of a user and asserts that the opening found by `reco`
//! is the same as the opening in Lichess.
//!
//! Panics if the latest game is a variant, or otherwise doesn't have an opening attached to it.

use pgn_reader::{RawTag, San, SanPlus, Skip, Visitor};
use shakmaty::{Chess, Position};

const USER: &str = "DrNykterstein";

#[derive(Default)]
struct SansAndOpening {
    pub sans: Vec<San>,
    pub opening: Option<String>,
}

impl Visitor for SansAndOpening {
    type Result = Self;

    fn tag(&mut self, name: &[u8], value: RawTag<'_>) {
        if name == b"Opening" {
            self.opening = Some(value.decode_utf8_lossy().to_string());
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn san(&mut self, san_plus: SanPlus) {
        self.sans.push(san_plus.san);
    }

    fn end_game(&mut self) -> Self::Result {
        std::mem::take(self)
    }
}

fn main() {
    let mut latest_games = ureq::get(format!("https://lichess.org/api/games/user/{USER}"))
        .query("max", "1")
        .query("opening", "true")
        .header(http::header::ACCEPT, "application/x-chess-pgn")
        .call()
        .unwrap()
        .into_body();

    let mut latest_games_reader = pgn_reader::BufferedReader::new(latest_games.as_reader());
    let SansAndOpening { sans, opening } = latest_games_reader
        .read_game(&mut SansAndOpening::default())
        .unwrap()
        .unwrap();
    let mut position = Chess::new();
    let mut moves = Vec::with_capacity(sans.len());

    for san in sans {
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

    assert_eq!(opening.unwrap(), reco_opening);
    println!("Opening finder correct");
}
