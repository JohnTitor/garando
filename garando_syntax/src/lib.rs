//! The Rust parser and macro expander.
//!
//! # Note
//!
//! This API is completely unstable and subject to change.

pub use garando_errors as errors;
use garando_pos as syntax_pos;
mod rustc_data_structures;

// A variant of 'try!' that panics on an Err. This is used as a crutch on the
// way towards a non-panic!-prone parser. It should be used for fatal parsing
// errors; eventually we plan to convert all code using panictry to just use
// normal try.
// Exported for syntax_ext, not meant for general use.
#[macro_export]
macro_rules! panictry {
    ($e:expr) => {{
        use crate::errors::FatalError;
        use std::result::Result::{Err, Ok};
        match $e {
            Ok(e) => e,
            Err(mut e) => {
                e.emit();
                panic!("{}", FatalError);
            }
        }
    }};
}

#[macro_export]
macro_rules! unwrap_or {
    ($opt:expr, $default:expr) => {
        match $opt {
            Some(x) => x,
            None => $default,
        }
    };
}

#[macro_use]
pub mod diagnostics {
    #[macro_use]
    pub mod macros;
    pub mod metadata;
    pub mod plugin;
}

// NB: This module needs to be declared first so diagnostics are
// registered before they are used.
pub mod diagnostic_list;

pub mod util {
    pub mod lev_distance;
    pub mod move_map;
    pub mod parser;
    #[cfg(test)]
    pub mod parser_testing;
    pub mod small_vector;

    mod thin_vec;
    pub use self::thin_vec::ThinVec;

    mod rc_slice;
    pub use self::rc_slice::RcSlice;
}

pub mod json;

pub mod syntax {
    pub use crate::ast;
    pub use crate::ext;
    pub use crate::parse;
}

pub mod abi;
pub mod ast;
pub mod attr;
pub mod codemap;
#[macro_use]
pub mod config;
pub mod entry;
pub mod feature_gate;
pub mod fold;
pub mod parse;
pub mod ptr;
pub mod show_span;
pub mod std_inject;
pub mod str;
pub use crate::syntax_pos::symbol;
pub mod test;
pub mod tokenstream;
pub mod visit;

pub mod print {
    pub mod pp;
    pub mod pprust;
}

pub mod ext {
    pub use crate::syntax_pos::hygiene;
    pub mod base;
    pub mod build;
    pub mod derive;
    pub mod expand;
    pub mod placeholders;
    pub mod quote;
    pub mod source_util;

    pub mod tt {
        pub mod macro_parser;
        pub mod macro_rules;
        pub mod quoted;
        pub mod transcribe;
    }
}

#[cfg(test)]
mod test_snippet;

// __build_diagnostic_array! { libsyntax, DIAGNOSTICS }
