#![forbid(unsafe_code)]

mod constants;
mod create_variation_files;
mod get_archive_and_commit;
mod get_line_expression_string;
mod get_name;
mod get_uci;
mod get_variation_item_string;

use crate::constants::{COMMIT_SOURCE_OUT, GEN_DIR};
use crate::create_variation_files::create_variation_files;
use crate::get_archive_and_commit::get_archive_and_commit;
use crate::get_name::get_name;
use crate::get_uci::get_uci;
use dotenvy::var;
use reco::Code;
use shakmaty::{Chess, EnPassantMode, Move, Position, Setup};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::{exists, remove_dir_all, write};
use std::io::Cursor;
use std::rc::Rc;
use std::str::FromStr;
use zip::ZipArchive;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LineMeta {
    pub code: Code,
    pub moves: Vec<Move>,
    pub setup: Setup,
}

pub struct VariationMeta {
    pub name: Rc<str>,
    pub variations: RefCell<HashMap<Rc<str>, Rc<Self>>>,
    pub parent: Option<Rc<Self>>,
    pub lines: RefCell<HashSet<LineMeta>>,
}

impl VariationMeta {
    pub fn full_name(&self) -> VecDeque<Rc<str>> {
        let mut full_name = VecDeque::with_capacity(10);

        full_name.push_front(self.name.clone());

        let mut parent = self.parent.clone();

        while let Some(current) = parent {
            full_name.push_front(current.name.clone());
            parent = current.parent.clone();
        }

        full_name
    }
}

/*
// sicilian/mod.rs

pub mod najdorf;
pub use najdorf::NAJDORF;

pub SICILIAN = Opening {
    name: "sicilian",
    variations: &[NAJDORF],
    parent: None
}

// sicilian/najdorf/mod.rs

pub mod classical;
pub use classical::CLASSICAL;

pub NAJDORF = Opening {
    name: "najdorf",
    variations: &[CLASSICAL],
    parent: Some(&super::SICILIAN)
}

// sicilian/najdorf/classical/mod.rs

pub CLASSICAL = Opening {
    name: "classical",
    variations: &[],
    parent: Some(&super::NAJDORF)
}
 */

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

    create_variation_files(variations);

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
    // for (identifier, opening) in &identifiers {
    //     let mut identifier = identifier.clone();
    //     // Removed because we want to work with the parent identifier
    //     let Some(last_variation) = identifier.pop() else {
    //         continue;
    //     };
    //
    //     if identifier.is_empty() {
    //         continue;
    //     }
    //
    //     let parent_directory_path = format!(
    //         "{GEN_DIR}/{}",
    //         identifier
    //             .iter()
    //             .map(|s| s.to_snek_case())
    //             .collect::<Vec<_>>()
    //             .join("/")
    //     );
    //
    //     if !exists(&parent_directory_path).unwrap() {
    //         create_dir_all(&parent_directory_path).unwrap();
    //     }
    //
    //     let parent_mod_path = format!("{parent_directory_path}/mod.rs");
    //
    //     let mut parent_mod = File::options()
    //         .create(true)
    //         .append(true)
    //         .open(&parent_mod_path)
    //         .unwrap_or_else(|_| panic!("{parent_mod_path} should exist"));
    //
    //     let r#use = if opening.is_some() {
    //         format!(
    //             "pub use {}::{};\n",
    //             last_variation.to_snek_case(),
    //             last_variation.TO_SHOUTY_SNEK_CASE()
    //         )
    //     } else {
    //         String::new()
    //     };
    //
    //     let mod_and_use = format!("pub mod {};\n{}", last_variation.to_snek_case(), r#use);
    //
    //     parent_mod.write_all(mod_and_use.as_bytes()).unwrap();
    // }

    // let top_level_opening_mods_and_uses = identifiers
    //     .iter()
    //     .filter_map(|(identifier, opening)| {
    //         if identifier.len() > 1 {
    //             None
    //         } else {
    //             let r#use = if opening.is_some() {
    //                 format!(
    //                     "pub use {}::{};\n",
    //                     identifier[0].to_snek_case(),
    //                     identifier[0].TO_SHOUTY_SNEK_CASE()
    //                 )
    //             } else {
    //                 String::new()
    //             };
    //
    //             Some(format!(
    //                 "pub mod {};\n{}",
    //                 identifier[0].to_snek_case(),
    //                 r#use
    //             ))
    //         }
    //     })
    //     .collect::<String>();
    //
    // let all_openings = identifiers
    //     .iter_mut()
    //     .filter_map(|(identifier, opening)| {
    //         if opening.is_none() {
    //             return None;
    //         }
    //
    //         let mut identifier = identifier.clone();
    //
    //         for part in identifier.iter_mut() {
    //             *part = part.to_snek_case();
    //         }
    //
    //         *identifier.last_mut().unwrap() = identifier.last().unwrap().TO_SHOUTY_SNEK_CASE();
    //         identifier[0] = format!("&{}", identifier[0]);
    //         Some(identifier.join("::"))
    //     })
    //     .collect::<Vec<_>>()
    //     .join(",\n");
    //
    // write(
    //     format!("{GEN_DIR}/mod.rs"),
    //     format!(
    //         "{BOOK_MOD_INIT}\
    //         {top_level_opening_mods_and_uses}\
    //         #[doc = \"Contains references to all openings and variations.\n\nThis is not a constant because it is huge, so inlining it is not desired.\nIt contains {openings_count} references, which is {} bytes on 64-bit systems.\"]
    //         pub static ALL: [&Opening<&str>; {openings_count}] = crate::concat_slices(&[{all_openings}]);",
    //         openings_count * size_of::<u64>()
    //     ).as_bytes()
    // )
    // .unwrap();
    write(COMMIT_SOURCE_OUT, commit_sha).unwrap();

    println!("success");
}
