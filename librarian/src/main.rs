#![forbid(unsafe_code)]

mod constants;
mod create_variation_files;
mod get_archive_and_commit;
mod get_line_expression_string;
mod get_name;
mod get_uci;
mod get_variation_item_string;
mod line_meta;
mod variation_meta;

use crate::constants::BOOK_MOD_INIT;
use crate::constants::{COMMIT_SOURCE_OUT, GEN_DIR};
use crate::create_variation_files::create_variation_files;
use crate::get_archive_and_commit::get_archive_and_commit;
use crate::get_name::get_name;
use crate::get_uci::get_uci;
use line_meta::LineMeta;
use reco::Code;
use shakmaty::{Chess, EnPassantMode, Position};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{exists, remove_dir_all, write};
use std::io::Cursor;
use std::rc::Rc;
use std::str::FromStr;
use variation_meta::VariationMeta;
use zip::ZipArchive;

fn main() {
    let (archive, commit_sha) = get_archive_and_commit();
    let mut archive = ZipArchive::new(Cursor::new(archive)).unwrap();
    let all_tsv = archive.by_name("all.tsv").unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .flexible(false)
        .from_reader(all_tsv);

    let mut variations = HashMap::new();

    for record in reader.records() {
        let record = record.unwrap();
        let code_raw = &record[0];
        let name_raw = &record[1];
        let uci_raw = &record[3];

        let code = Code::from_str(code_raw).unwrap();
        let mut full_name = get_name(name_raw);

        assert!(!full_name.is_empty());

        let uci = get_uci(uci_raw);
        let mut moves = Vec::new();
        let mut p = Chess::new();

        for uci in &uci {
            let r#move = uci.to_move(&p).unwrap();
            p = p.play(r#move).unwrap();
            moves.push(r#move);
        }

        let setup = p.to_setup(EnPassantMode::Legal);
        let root = full_name.remove(0);

        // Create root if it doesn't exist
        let mut variation = variations
            .entry(root)
            .or_insert_with_key(|k| {
                Rc::new(VariationMeta {
                    name: k.clone(),
                    variations: RefCell::new(HashMap::new()),
                    parent: None,
                    lines: RefCell::new(HashSet::new()),
                })
            })
            .clone();

        for part in full_name {
            let subvariation = variation
                .variations
                .borrow_mut()
                // For each part (except first), look for a subvariation.
                .entry(part)
                // If it's vacant, insert an empty variation with `variation` parent.
                .or_insert_with_key(|k| {
                    Rc::new(VariationMeta {
                        name: k.clone(),
                        variations: RefCell::new(HashMap::new()),
                        parent: Some(variation.clone()),
                        lines: RefCell::new(HashSet::new()),
                    })
                })
                .clone();

            variation = subvariation;
        }

        variation
            .lines
            .borrow_mut()
            .insert(LineMeta { code, moves, setup });
    }

    // Delete previous data
    if exists(GEN_DIR).unwrap() {
        remove_dir_all(GEN_DIR).unwrap();
    }

    create_variation_files(&variations);

    let root_mods_and_uses = variations
        .values()
        .map(|variation| {
            format!(
                "pub mod {0};\npub use {0}::{1};\n",
                variation.snek_name(),
                variation.SNEK_NAME()
            )
        })
        .collect::<String>();

    let root_identifiers = variations
        .values()
        .map(|v| format!("&{}", v.SNEK_NAME()))
        .collect::<Vec<_>>();

    write(COMMIT_SOURCE_OUT, commit_sha).unwrap();
    write(
        format!("{GEN_DIR}/mod.rs"),
        format!(
            r#"{BOOK_MOD_INIT}
{root_mods_and_uses}
/// All root variations.
pub static ALL: [&'static Variation; {}] = [{}];"#,
            root_identifiers.len(),
            root_identifiers.join(",\n")
        )
        .as_bytes(),
    )
    .unwrap();

    println!("success");
}
