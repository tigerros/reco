use crate::VariationMeta;
use crate::get_line_expression_string::get_line_expression_string;

/// Returns a string that represents the item of the given variation.
pub fn get_variation_item_string(variation: &VariationMeta) -> String {
    let name = &variation.name;

    let parent = if let Some(parent) = &variation.parent {
        format!("Some(&super::{})", parent.name)
    } else {
        "None".to_string()
    };

    let variations = variation
        .variations
        .borrow()
        .iter()
        .map(|(name, variation)| {
            assert_eq!(name, &variation.name);

            format!("&{name}")
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

    format!(
        r#"Variation {{
        name: {name},
        parent: {parent},
        variations: &[{variations}],
        lines: &[{lines}]
    }}"#
    )
}
