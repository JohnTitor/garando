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
    /// Reports an obsolete syntax non-fatal error.
    fn obsolete(&mut self, sp: Span, kind: ObsoleteSyntax);
    fn report(&mut self, sp: Span, kind: ObsoleteSyntax, kind_str: &str, desc: &str, error: bool);
}

impl<'a> ParserObsoleteMethods for parser::Parser<'a> {
    /// Reports an obsolete syntax non-fatal error.
    #[allow(unused_variables)]
    #[allow(unreachable_code)]
    fn obsolete(&mut self, sp: Span, kind: ObsoleteSyntax) {
        let (kind_str, desc, error) = match kind {
            // Nothing here at the moment
        };

        self.report(sp, kind, kind_str, desc, error);
    }

    fn report(&mut self, sp: Span, kind: ObsoleteSyntax, kind_str: &str, desc: &str, error: bool) {
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
            err.note(desc);
            self.obsolete_set.insert(kind);
        }
        err.emit();
    }
}
