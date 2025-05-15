#![forbid(unsafe_code)]

mod constants;
mod get_archive_and_commit;
mod get_name_and_variation;
mod get_opening_constant_expression_string;
mod get_opening_constant_item_string;
mod get_uci;

use crate::constants::{COMMIT_SOURCE_OUT, GEN_DIR};
use crate::get_archive_and_commit::get_archive_and_commit;
use crate::get_name_and_variation::get_name_and_variation;
use crate::get_opening_constant_expression_string::get_opening_constant_expression_string;
use crate::get_opening_constant_item_string::get_opening_constant_item_string;
use crate::get_uci::get_uci;
use deunicode::deunicode;
use heck::{ToShoutySnekCase, ToSnekCase};
use reco::{Code, Opening, OpeningOwned};
use shakmaty::{Chess, EnPassantMode, Position};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, create_dir_all, exists, remove_dir_all, write};
use std::io::{Cursor, Write};
use std::str::FromStr;
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
    let mut identifiers = BTreeMap::<Vec<String>, Option<(String, BTreeSet<String>)>>::new();
    let mut openings_count = 0;

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

        openings_count += 1;

        // Insert an identifier for each possible variation
        // For example, if identifier is `a, b, c`, this will insert
        // `a`, `a, b` and `a, b, c` into identifiers.
        for i in 1..=identifier.len() {
            let _ = identifiers.entry(identifier[0..i].to_vec()).or_insert(None);
        }

        let value = identifiers.get_mut(&identifier).unwrap();

        if let Some((full_name, silent_variations)) = value {
            *full_name = full_name_raw.to_owned();
            silent_variations.insert(get_opening_constant_expression_string(&OpeningOwned {
                code,
                name,
                variation,
                moves,
                setup,
            }));
        } else {
            *value = Some((
                full_name_raw.to_owned(),
                BTreeSet::from([get_opening_constant_expression_string(&OpeningOwned {
                    code,
                    name,
                    variation,
                    moves,
                    setup,
                })]),
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
            use crate::Opening;\
            {top_level_opening_mods_and_uses}\
            #[doc = \"Contains all openings and variations.\n\nThis is a static and not a constant because it's huge.\nIt contains {openings_count} openings, which is {} bytes.\"]
            pub static ALL: &[Opening<'static, &str>] = constcat::concat_slices!([Opening<'static, &str>]: {all_openings});",
            openings_count * size_of::<Opening<'static, &str>>()
        ).as_bytes()
    )
    .unwrap();
    write(COMMIT_SOURCE_OUT, commit_sha).unwrap();

    println!("success");
}
