use heck::ToShoutySnekCase;
use itertools::Itertools;
use std::collections::BTreeSet;

/// Returns a string of a constant item declaration of an array of the `silent_variations`.
///
/// `identifier` is converted to SHOUTY_SNEK_CASE.
pub fn get_opening_constant_item_string(
    identifier: &str,
    full_name: &str,
    silent_variations: &BTreeSet<String>,
) -> String {
    format!(
        "\n\n/// {full_name}.\npub const {}: [Opening<'static, &str>; {}] = [{}];",
        identifier.TO_SHOUTY_SNEK_CASE(),
        silent_variations.len(),
        silent_variations.iter().join(", ")
    )
}
