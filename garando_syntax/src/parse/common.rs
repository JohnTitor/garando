//! Common routines shared by parser mods

use crate::parse::token;

/// `SeqSep` : a sequence separator (token)
/// and whether a trailing separator is allowed.
pub struct SeqSep {
    pub sep: Option<token::Token>,
    pub trailing_sep_allowed: bool,
}

impl SeqSep {
    pub fn trailing_allowed(t: token::Token) -> SeqSep {
        SeqSep {
            sep: Some(t),
            trailing_sep_allowed: true,
        }
    }

    pub fn none() -> SeqSep {
        SeqSep {
            sep: None,
            trailing_sep_allowed: false,
        }
    }
}
