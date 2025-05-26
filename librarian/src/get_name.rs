use std::rc::Rc;

/// `name_raw` should be in the form `Opening: Var1, Var2, ...`.
pub fn get_name(name_raw: &str) -> Vec<Rc<str>> {
    let name_raw = name_raw.replacen(':', ",", 1);
    name_raw
        .split(',')
        .map(str::trim)
        .map(Rc::from)
        .collect::<Vec<_>>()
}
