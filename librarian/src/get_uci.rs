use shakmaty::uci::UciMove;
use std::str::FromStr;

/// Extracts a list of [`UciMove`]s from a string of UCI moves separated by spaces.
///
/// `raw` should be in the form `e2e4 c7c5 ...`.
pub fn get_uci(raw: &str) -> Vec<UciMove> {
    raw.split(' ')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(UciMove::from_str(s).unwrap())
            }
        })
        .collect::<Vec<_>>()
}
