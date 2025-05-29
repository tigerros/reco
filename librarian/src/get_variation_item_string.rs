use crate::VariationMeta;
use crate::get_line_expression_string::get_line_expression_string;

/// Returns a string that represents the item of the given variation.
pub fn get_variation_item_string(variation: &VariationMeta) -> String {
    let original_name = variation.original_name();
    let identifier = variation.SNEK_NAME();
    let name = &variation.name;

    let parent = if let Some(parent) = &variation.parent {
        format!("Some(&super::{})", parent.SNEK_NAME())
    } else {
        "None".to_string()
    };

    let variations = variation
        .variations
        .borrow()
        .iter()
        .map(|(name, variation)| {
            assert_eq!(name, &variation.name);

            format!("&{}", variation.SNEK_NAME())
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let lines = variation
        .lines
        .borrow()
        .iter()
        .map(get_line_expression_string)
        .collect::<Vec<_>>()
        .join(",\n");

    // Get the module path
    let path = variation
        .full_snek_name()
        .into_iter()
        .rev()
        .skip(1)
        .rev()
        .map(|s| format!("{s}::"))
        .collect::<String>();

    format!(
        r##"#[cfg_attr(feature = "alloc", doc = r#"```rust
# use reco::book::{path}{identifier};
assert_eq!({identifier}.original_name(), "{original_name}");
```"#)]
pub static {identifier}: Variation = Variation {{
    name: "{name}",
    variations: &[{variations}],
    parent: {parent},
    lines: &[{lines}]
}};"##
    )
}
