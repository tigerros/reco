#![forbid(unsafe_code)]

mod constants;

use crate::constants::{BRANCH, GEN_DIR, OWNER, REPO, WORKFLOW};
use deunicode::deunicode;
use heck::{ToShoutySnekCase, ToSnekCase};
use http::header;
use reco_core::{Code, Opening, OpeningOwned};
use serde_json::Value;
use shakmaty::uci::UciMove;
use shakmaty::{Chess, EnPassantMode, Position};
use std::collections::HashMap;
use std::fs::{File, create_dir_all, exists, remove_dir_all, write};
use std::io::{Cursor, Write};
use std::str::FromStr;
use std::time::Duration;
use ureq::Agent;
use zip::ZipArchive;

fn main() {
    println!("commit anything in the openings directory before running this script");
    println!("only commit results if this script prints out \"success\" and reco compiles");

    let (archive, commit_sha) = get_archive_and_commit();
    let mut archive = ZipArchive::new(Cursor::new(archive)).unwrap();
    let all_tsv = archive.by_name("all.tsv").unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .flexible(false)
        .from_reader(all_tsv);

    // The key is the full identifier of the opening.
    // A `None` value means the identifier is not the name of an opening.
    // For example, pterodactlyl_defense::western is only a group of openings.
    // That identifier would be `None`.
    // The first value is the original full name of the opening.
    // The second value are all the "silent variations", those being different entries but having the
    // same identifier. Each item is already a string of a const expression.
    let mut identifiers = HashMap::<Vec<String>, Option<(String, Vec<String>)>>::new();

    for record in reader.records() {
        let record = record.unwrap();
        let code_raw = &record[0];
        let full_name_raw = &record[1];
        let uci_raw = &record[3];

        let code = Code::from_str(code_raw).unwrap();
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

        let mut identifier = variation.clone();
        identifier.insert(0, name.clone());

        for part in identifier.iter_mut() {
            *part = deunicode(part);
            // Queen's Gambit should be converted to QUEENS_GAMBIT, not QUEEN_S_GAMBIT
            *part = part.replace('\'', "");
            *part = part.trim().to_owned();

            if part.starts_with(char::is_numeric) {
                part.insert(0, 'N');
            }
        }

        // Insert an identifier for each possible variation
        // For example, if identifier is `a, b, c`, this will insert
        // `a`, `a, b` and `a, b, c` into identifiers.
        for i in 1..=identifier.len() {
            let _ = identifiers.entry(identifier[0..i].to_vec()).or_insert(None);
        }

        let value = identifiers.get_mut(&identifier).unwrap();

        if let Some((full_name, silent_variations)) = value {
            *full_name = full_name_raw.to_owned();
            silent_variations.push(get_opening_constant_expression_string(&OpeningOwned {
                code,
                name,
                variation,
                moves,
                setup,
            }));
        } else {
            *value = Some((
                full_name_raw.to_owned(),
                vec![get_opening_constant_expression_string(&OpeningOwned {
                    code,
                    name,
                    variation,
                    moves,
                    setup,
                })],
            ));
        }
    }

    // Delete previous data
    if exists(GEN_DIR).unwrap() {
        remove_dir_all(GEN_DIR).unwrap();
    }

    // Creates a directory for each opening and a module file, where the opening is stored.
    for (identifier, (full_name, silent_variations)) in identifiers
        .iter()
        .filter_map(|(identifier, opening)| opening.as_ref().map(|opening| (identifier, opening)))
    {
        let directory_path = format!(
            "{GEN_DIR}/{}",
            identifier
                .iter()
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

        let mut file = File::options().append(true).open(file_path).unwrap();

        file.write_all(
            get_opening_constant_item_string(
                identifier.last().unwrap(),
                full_name,
                silent_variations,
            )
            .as_bytes(),
        )
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
    for (identifier, opening) in &identifiers {
        let mut identifier = identifier.clone();
        // Removed because we want to work with the parent identifier
        let Some(last_variation) = identifier.pop() else {
            continue;
        };

        if identifier.is_empty() {
            continue;
        }

        let parent_directory_path = format!(
            "{GEN_DIR}/{}",
            identifier
                .iter()
                .map(|s| s.to_snek_case())
                .collect::<Vec<_>>()
                .join("/")
        );

        if !exists(&parent_directory_path).unwrap() {
            create_dir_all(&parent_directory_path).unwrap();
        }

        let parent_mod_path = format!("{parent_directory_path}/mod.rs");

        let mut parent_mod = File::options()
            .create(true)
            .append(true)
            .open(&parent_mod_path)
            .unwrap_or_else(|_| panic!("{parent_mod_path} should exist"));

        let r#use = if opening.is_some() {
            format!(
                "pub use {}::{};\n",
                last_variation.to_snek_case(),
                last_variation.TO_SHOUTY_SNEK_CASE()
            )
        } else {
            String::new()
        };

        let mod_and_use = format!("pub mod {};\n{}", last_variation.to_snek_case(), r#use);

        parent_mod.write_all(mod_and_use.as_bytes()).unwrap();
    }

    let top_level_opening_mods_and_uses = identifiers
        .iter()
        .filter_map(|(identifier, opening)| {
            if identifier.len() > 1 {
                None
            } else {
                let r#use = if opening.is_some() {
                    format!(
                        "pub use {}::{};\n",
                        identifier[0].to_snek_case(),
                        identifier[0].TO_SHOUTY_SNEK_CASE()
                    )
                } else {
                    String::new()
                };

                Some(format!(
                    "pub mod {};\n{}",
                    identifier[0].to_snek_case(),
                    r#use
                ))
            }
        })
        .collect::<String>();

    let all_openings = identifiers
        .iter_mut()
        .filter_map(|(identifier, opening)| {
            if opening.is_none() {
                return None;
            }

            let mut identifier = identifier.clone();

            for part in identifier.iter_mut() {
                *part = part.to_snek_case();
            }

            *identifier.last_mut().unwrap() = identifier.last().unwrap().TO_SHOUTY_SNEK_CASE();
            identifier[0] = format!("&{}", identifier[0]);
            Some(identifier.join("::"))
        })
        .collect::<Vec<_>>()
        .join(",\n");

    write(
        format!("{GEN_DIR}/mod.rs"),
        format!(
            "#![allow(\
                clippy::allow_attributes,\
                reason = \"this module is generated, the allows don't know if they are going to be fulfilled\"\
            )]\
            use reco_core::Opening;\
            {top_level_opening_mods_and_uses}\
            /// Contains all openings and variations.\n
            /// \n
            /// This is a static and not a constant because it's huge.\n
            /// It contains roughly 3500 openings, which is {} bytes.
            pub static ALL: &[Opening<'static, &str>] = constcat::concat_slices!([Opening<'static, &str>]: {all_openings});",
            3500 * size_of::<Opening<'static, &str>>()
        ).as_bytes()
    )
    .unwrap();
    write("book-commit-source.txt", commit_sha).unwrap();

    println!("success");
}

/// Returns a string of a constant item declaration of an array of the `silent_variations`.
///
/// `identifier` is converted to SHOUTY_SNEK_CASE.
fn get_opening_constant_item_string(
    identifier: &str,
    full_name: &str,
    silent_variations: &[String],
) -> String {
    format!(
        "\n\n/// {full_name}.\npub const {}: [Opening<'static, &str>; {}] = [{}];",
        identifier.TO_SHOUTY_SNEK_CASE(),
        silent_variations.len(),
        silent_variations.join(", ")
    )
}

/// Returns a string of a const expression which represents the given opening.
///
/// Uses the last variation layer for the constant identifier, or the opening name if the
/// variation is unspecified.
fn get_opening_constant_expression_string(opening: &OpeningOwned) -> String {
    let name = &opening.name;
    let moves = &opening.moves;
    let volume = opening.code.volume;
    let category = opening.code.category;
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
        r#"Opening {{
    code: Code {{
        volume: Volume::{volume},
        category: RangedU8::new_static::<{category}>(),
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
        "https://api.github.com/repos/{OWNER}/{REPO}/actions/workflows/{WORKFLOW}/runs?branch={BRANCH}"
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
