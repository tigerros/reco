use crate::GEN_DIR;
use crate::{VariationMeta, constants};
use deunicode::deunicode;
use heck::ToSnekCase;
use std::collections::BTreeMap;
use std::fs::{File, create_dir_all, exists, write};
use std::io::Write;
use std::rc::Rc;

/// Creates a directory for each variation and a module file where the variation is stored.
pub fn create_variation_files(variations: &BTreeMap<Rc<str>, Rc<VariationMeta>>) {
    for variation in variations.values() {
        let full_name = variation.full_snek_name();

        let directory_path = format!(
            "{GEN_DIR}/{}",
            full_name
                .iter()
                .map(|s| deunicode(s).to_snek_case())
                .collect::<Vec<_>>()
                .join("/")
        );

        if !exists(&directory_path).unwrap() {
            create_dir_all(&directory_path).unwrap();
        }

        let file_path = format!("{directory_path}/mod.rs");

        if !exists(&file_path).unwrap() {
            write(&file_path, constants::VARIATION_FILE_INIT).unwrap();
        }

        let mut file = File::options().append(true).open(file_path).unwrap();

        file.write_all(variation.item_string().as_bytes()).unwrap();

        let subvariations = variation.variations.borrow();
        let mods_and_uses = subvariations
            .values()
            .map(|subvariation| {
                format!(
                    "pub mod {0};\npub use {0}::{1};\n",
                    subvariation.snek_name(),
                    subvariation.SNEK_NAME(),
                )
            })
            .collect::<String>();

        file.write_all(mods_and_uses.as_bytes()).unwrap();

        create_variation_files(&subvariations);
    }
}
