#![forbid(unsafe_code)]

mod constants;

use heck::{ToShoutySnekCase, ToSnekCase};
use http::header;
use reco::OpeningOwned;
use serde_json::Value;
use shakmaty::uci::UciMove;
use shakmaty::{Chess, EnPassantMode, Position};
use std::fs::{File, create_dir_all, exists, remove_dir_all, write};
use std::io::{Cursor, Write};
use std::iter;
use std::str::FromStr;
use std::time::Duration;
use ureq::Agent;
use zip::ZipArchive;

fn main() {
    println!("commit anything in the volume directories before running this script");
    println!("only commit results if this script prints out \"success\"");

    let (archive, commit_sha) = get_archive_and_commit();
    let mut archive = ZipArchive::new(Cursor::new(archive)).unwrap();
    let all_tsv = archive.by_name("all.tsv").unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .flexible(false)
        .from_reader(all_tsv);

    let mut openings = Vec::new();

    for record in reader.records() {
        let record = record.unwrap();
        let code_raw = &record[0];
        let full_name_raw = &record[1];
        let uci_raw = &record[3];

        let code = reco::Code::from_str(code_raw).unwrap();
        let (name, variation) = get_name_and_variation(full_name_raw);
        let uci = get_uci(uci_raw);
        let mut moves = Vec::new();
        let mut p = Chess::new();

        for uci in &uci {
            let r#move = uci.to_move(&p).unwrap();
            p = p.play(r#move).unwrap();
            moves.push(r#move);
        }

        let setup = p.to_setup(EnPassantMode::Legal);

        openings.push(OpeningOwned {
            code,
            name,
            variation,
            moves,
            setup,
        });
    }

    // Delete previous data
    if exists("src/openings").unwrap() {
        remove_dir_all("src/openings").unwrap();
    }

    // Creates a directory for each opening and a module file, where the opening is stored.
    for opening in &openings {
        let directory_path = format!(
            "src/openings/{}",
            iter::once(&opening.name)
                .chain(&opening.variation)
                .map(|s| s.to_snek_case())
                .collect::<Vec<_>>()
                .join("/")
        );

        if !exists(&directory_path).unwrap() {
            create_dir_all(&directory_path).unwrap();
        }

        let file_path = format!("{directory_path}/mod.rs");

        if !exists(&file_path).unwrap() {
            write(&file_path, constants::OPENING_FILE_INIT).unwrap();
        }

        let mut file = File::options()
            .write(true)
            .append(true)
            .open(file_path)
            .unwrap();

        file.write_all(get_opening_constant_string(opening).as_bytes())
            .unwrap();
    }

    // Each opening adds itself to the module file to the parent.
    // Example:
    // sicilian/
    // > mod.rs
    // > najdorf/...
    //
    // If this loop encounters "sicilian", nothing happens (empty variation).
    // But when it encounters "sicilian: najdorf", it will append
    // `pub mod najdorf; pub use najdorf::NAJDORF;` to `sicilian/mod.rs`.
    // Unfortunately it will be written after the `sicilian::SICILIAN` constant item.
    // I don't see it as a huge problem because the files are generated anyway,
    // and doing it another way would probably make this script more complex.
    for opening in &mut openings {
        // Removed because we want to work with the parent opening
        let Some(last_variation) = opening.variation.pop() else {
            continue;
        };

        let parent_mod_path = format!(
            "src/openings/{}/{}/mod.rs",
            opening.name.to_snek_case(),
            opening
                .variation
                .iter()
                .map(|s| s.to_snek_case())
                .collect::<Vec<_>>()
                .join("/")
        );

        let mut parent_mod = File::options().append(true).open(parent_mod_path).unwrap();

        let mod_and_use = format!(
            "pub mod {0};\npub use {0}::{1}",
            last_variation.to_snek_case(),
            last_variation.TO_SHOUTY_SNEK_CASE()
        );

        parent_mod.write_all(mod_and_use.as_bytes()).unwrap();
    }

    let top_level_opening_mods_and_uses = openings
        .iter()
        .filter_map(|opening| {
            if opening.variation.is_empty() {
                None
            } else {
                Some(format!(
                    "pub mod {0};\npub use {0}::{1};\n",
                    opening.name.to_snek_case(),
                    opening.name.TO_SHOUTY_SNEK_CASE()
                ))
            }
        })
        .collect::<String>();

    write(
        "src/openings/mod.rs",
        top_level_opening_mods_and_uses.as_bytes(),
    )
    .unwrap();
    write("commit-run-source.txt", commit_sha).unwrap();

    println!("success");
}

/// Returns an item which is a constant of the given opening.
///
/// Uses the last variation layer for the constant identifier, or the opening name if the
/// variation is unspecified.
fn get_opening_constant_string(opening: &OpeningOwned) -> String {
    let name = &opening.name;
    let moves = &opening.moves;
    let identifier = opening
        .variation
        .last()
        .unwrap_or(&opening.name)
        .TO_SHOUTY_SNEK_CASE();
    let volume = opening.code.volume;
    let subcategory = opening.code.subcategory;
    let variation_joined = opening
        .variation
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
        r#"

pub const {identifier}: Opening<'static, &str> = Opening {{
    code: Code {{
        volume: Volume::{volume},
        subcategory: RangedU8::new_static::<{subcategory}>(),
    }},
    name: "{name}",
    variation: &[{variation_joined}],
    moves: &{moves:#?},
    setup: &Setup {{
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
    }},
}};"#,
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

fn get_name_and_variation(full_name: &str) -> (String, Vec<String>) {
    let full_name_split = full_name.split(':').collect::<Vec<_>>();
    let name = full_name_split[0];

    if full_name_split.len() == 1 {
        return (name.to_owned(), Vec::new());
    }

    let variation = full_name_split[1]
        .split(',')
        .map(str::to_owned)
        .collect::<Vec<_>>();

    (name.to_owned(), variation)
}

fn get_uci(raw: &str) -> Vec<UciMove> {
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

fn get_archive_and_commit() -> (Vec<u8>, String) {
    let token = format!("Bearer {}", dotenvy::var("GITHUB_TOKEN").unwrap());

    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .user_agent("tigerros/reco")
        .build()
        .into();

    let runs_url = format!(
        "https://api.github.com/repos/{}/{}/actions/workflows/{}/runs?branch={}",
        constants::OWNER,
        constants::REPO,
        constants::WORKFLOW,
        constants::BRANCH,
    );

    let runs_res: Value = agent
        .get(runs_url)
        .header(header::AUTHORIZATION, &token)
        .call()
        .unwrap()
        .into_body()
        .read_json()
        .unwrap();

    let mut archive_download_url_and_commit_sha = None::<(String, String)>;

    for run in runs_res["workflow_runs"].as_array().unwrap() {
        if run["name"] != "Lint" || run["conclusion"] != "success" || run["event"] != "push" {
            continue;
        }

        let artifacts_url = run["artifacts_url"].as_str().unwrap();

        let artifacts_res: Value = agent
            .get(artifacts_url)
            .header(header::AUTHORIZATION, &token)
            .call()
            .unwrap()
            .into_body()
            .read_json()
            .unwrap();

        if let Some(artifact) = artifacts_res["artifacts"].as_array().unwrap().first() {
            archive_download_url_and_commit_sha = Some((
                artifact["archive_download_url"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                run["head_sha"].as_str().unwrap().to_owned(),
            ));
            break;
        };
    }

    let Some((archive_download_url, commit_sha)) = archive_download_url_and_commit_sha else {
        panic!("no artifact found");
    };

    let archive_res = agent
        .get(archive_download_url)
        .header(header::AUTHORIZATION, &token)
        .call()
        .unwrap()
        .into_body()
        .into_with_config()
        .limit(u64::MAX)
        .read_to_vec()
        .unwrap();

    (archive_res, commit_sha)
}
