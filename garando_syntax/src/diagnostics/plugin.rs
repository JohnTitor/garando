use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::ast::Name;
use crate::ext::base::{ExtCtxt, MacEager, MacResult};
use crate::ext::build::AstBuilder;
use crate::parse::token;
use crate::syntax_pos::Span;
use crate::tokenstream::TokenTree;

pub use crate::errors::*;

thread_local! {
    static REGISTERED_DIAGNOSTICS: RefCell<ErrorMap> = {
        RefCell::new(BTreeMap::new())
    }
}

/// Error information type.
pub struct ErrorInfo {
    pub description: Option<Name>,
    pub use_site: Option<Span>,
}

/// Mapping from error codes to metadata.
pub type ErrorMap = BTreeMap<Name, ErrorInfo>;

fn with_registered_diagnostics<T, F>(f: F) -> T
where
    F: FnOnce(&mut ErrorMap) -> T,
{
    REGISTERED_DIAGNOSTICS.with(move |slot| f(&mut *slot.borrow_mut()))
}

pub fn expand_diagnostic_used<'cx>(
    ecx: &'cx mut ExtCtxt,
    span: Span,
    token_tree: &[TokenTree],
) -> Box<dyn MacResult + 'cx> {
    let code = match (token_tree.len(), token_tree.get(0)) {
        (1, Some(&TokenTree::Token(_, token::Ident(code)))) => code,
        _ => unreachable!(),
    };

    with_registered_diagnostics(|diagnostics| {
        match diagnostics.get_mut(&code.name) {
            // Previously used errors.
            Some(&mut ErrorInfo {
                description: _,
                use_site: Some(previous_span),
            }) => {
                ecx.struct_span_warn(span, &format!("diagnostic code {} already used", code))
                    .span_note(previous_span, "previous invocation")
                    .emit();
            }
            // Newly used errors.
            Some(ref mut info) => {
                info.use_site = Some(span);
            }
            // Unregistered errors.
            None => {
                ecx.span_err(
                    span,
                    &format!("used diagnostic code {} not registered", code),
                );
            }
        }
    });
    MacEager::expr(ecx.expr_tuple(span, Vec::new()))
}
