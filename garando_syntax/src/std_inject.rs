use crate::{ast, attr};

pub fn injected_crate_name(krate: &ast::Crate) -> Option<&'static str> {
    if attr::contains_name(&krate.attrs, "no_core") {
        None
    } else if attr::contains_name(&krate.attrs, "no_std") {
        Some("core")
    } else {
        Some("std")
    }
}
