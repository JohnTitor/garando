//! Support for parsing unsupported, old syntaxes, for the purpose of reporting errors. Parsing of
//! these syntaxes is tested by compile-test/obsolete-syntax.rs.
//!
//! Obsolete syntax that becomes too hard to parse can be removed.

use crate::parse::parser;
use crate::syntax_pos::Span;

/// The specific types of unsupported syntax
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ObsoleteSyntax {
    // Nothing here at the moment
}

pub trait ParserObsoleteMethods {
    fn report(&mut self, sp: Span, kind: ObsoleteSyntax, kind_str: &str, error: bool);
}

impl<'a> ParserObsoleteMethods for parser::Parser<'a> {
    fn report(&mut self, sp: Span, kind: ObsoleteSyntax, kind_str: &str, error: bool) {
        let mut err = if error {
            self.diagnostic()
                .struct_span_err(sp, &format!("obsolete syntax: {}", kind_str))
        } else {
            self.diagnostic()
                .struct_span_warn(sp, &format!("obsolete syntax: {}", kind_str))
        };

        if !self.obsolete_set.contains(&kind)
            && (error || self.sess.span_diagnostic.can_emit_warnings)
        {
            self.obsolete_set.insert(kind);
        }
        err.emit();
    }
}
