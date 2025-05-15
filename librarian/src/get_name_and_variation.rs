/// Extracts the top-level name and variation from the full name.
///
/// `full_name` should be in the form `Opening: Var1, Var2, ...`.
pub fn get_name_and_variation(full_name: &str) -> (String, Vec<String>) {
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
