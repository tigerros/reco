#![forbid(unsafe_code)]

mod constants;

use heck::ToShoutySnakeCase;
use http::header;
use reco::Volume;
use serde_json::Value;
use shakmaty::uci::UciMove;
use shakmaty::{Chess, EnPassantMode, Position};
use std::collections::{HashMap, HashSet};
use std::fs::{create_dir_all, read_dir, remove_dir_all, remove_file, write};
use std::io::Cursor;
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

    let mut codes = HashSet::new();
    let mut files = HashMap::new();

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
        let file_path = format!("src/{}/{code}.rs", code.volume);

        let file = files.entry(file_path).or_insert_with(|| {
            r#"#[expect(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[expect(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[expect(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
use shakmaty::Square::*;
#[expect(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByRole, ByColor, Setup};
use core::num::NonZeroU32;
use crate::{Opening, Code, Volume};
use deranged::RangedU8;"#
                .to_owned()
        });

        let identifier = full_name_raw.to_shouty_snake_case();
        let volume = code.volume;
        let subcategory = code.subcategory;
        let variation_joined = variation
            .iter()
            .map(|v| format!("\"{}\"", v.trim()))
            .collect::<Vec<_>>()
            .join(", ");
        let (by_role_bitboard, by_color_bitboard) = setup.board.into_bitboards();
        let promoted = setup.promoted.0;
        let pockets = setup.pockets;
        let turn = setup.turn;
        let castling_rights = setup.castling_rights.0;
        let ep_square = setup.ep_square;
        let remaining_checks = setup.remaining_checks;
        let halfmoves = setup.halfmoves;
        let fullmoves = setup.fullmoves;

        let s = format!(
            r#"

pub const {identifier}: Opening<'static, &str, &str> = Opening {{
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
        );

        file.push_str(&s);
        codes.insert(code);
    }

    for volume in Volume::ALL {
        let dir = format!("src/{volume}");
        create_dir_all(&dir).unwrap();

        for entry in read_dir(dir).unwrap() {
            let entry = entry.unwrap();

            if entry.path().is_file() {
                remove_file(entry.path()).unwrap();
            } else if entry.path().is_dir() {
                remove_dir_all(entry.path()).unwrap();
            }
        }
    }

    for (file_path, file) in files {
        write(file_path, file).unwrap();
    }

    for volume in Volume::ALL {
        write(
            format!("src/{volume}/mod.rs"),
            codes
                .iter()
                .filter_map(|c| {
                    if c.volume == volume {
                        Some(format!("pub mod {c};"))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap();
    }

    write("commit-run-source.txt", commit_sha).unwrap();

    println!("success");
}

fn get_name_and_variation(full_name: &str) -> (&str, Vec<&str>) {
    let full_name_split = full_name.split(':').collect::<Vec<_>>();
    let name = full_name_split[0];

    if full_name_split.len() == 1 {
        return (name, Vec::new());
    }

    let variation = full_name_split[1].split(',').collect::<Vec<_>>();

    (name, variation)
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
        if !(run["name"] == "Lint") || !(run["conclusion"] == "success") {
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
