use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Abi {
    // NB: This ordering MUST match the AbiDatas array below.
    // (This is ensured by the test indices_are_correct().)

    // Single platform ABIs
    Cdecl,
    Stdcall,
    Fastcall,
    Vectorcall,
    Thiscall,
    Aapcs,
    Win64,
    SysV64,
    PtxKernel,
    Msp430Interrupt,
    X86Interrupt,

    // Multiplatform / generic ABIs
    Rust,
    C,
    System,
    RustIntrinsic,
    RustCall,
    PlatformIntrinsic,
    Unadjusted,
}

#[derive(Copy, Clone)]
pub struct AbiData {
    abi: Abi,

    /// Name of this ABI as we like it called.
    name: &'static str,
}

#[allow(non_upper_case_globals)]
const AbiDatas: &'static [AbiData] = &[
    // Platform-specific ABIs
    AbiData {
        abi: Abi::Cdecl,
        name: "cdecl",
    },
    AbiData {
        abi: Abi::Stdcall,
        name: "stdcall",
    },
    AbiData {
        abi: Abi::Fastcall,
        name: "fastcall",
    },
    AbiData {
        abi: Abi::Vectorcall,
        name: "vectorcall",
    },
    AbiData {
        abi: Abi::Thiscall,
        name: "thiscall",
    },
    AbiData {
        abi: Abi::Aapcs,
        name: "aapcs",
    },
    AbiData {
        abi: Abi::Win64,
        name: "win64",
    },
    AbiData {
        abi: Abi::SysV64,
        name: "sysv64",
    },
    AbiData {
        abi: Abi::PtxKernel,
        name: "ptx-kernel",
    },
    AbiData {
        abi: Abi::Msp430Interrupt,
        name: "msp430-interrupt",
    },
    AbiData {
        abi: Abi::X86Interrupt,
        name: "x86-interrupt",
    },
    // Cross-platform ABIs
    AbiData {
        abi: Abi::Rust,
        name: "Rust",
    },
    AbiData {
        abi: Abi::C,
        name: "C",
    },
    AbiData {
        abi: Abi::System,
        name: "system",
    },
    AbiData {
        abi: Abi::RustIntrinsic,
        name: "rust-intrinsic",
    },
    AbiData {
        abi: Abi::RustCall,
        name: "rust-call",
    },
    AbiData {
        abi: Abi::PlatformIntrinsic,
        name: "platform-intrinsic",
    },
    AbiData {
        abi: Abi::Unadjusted,
        name: "unadjusted",
    },
];

/// Returns the ABI with the given name (if any).
pub fn lookup(name: &str) -> Option<Abi> {
    AbiDatas
        .iter()
        .find(|abi_data| name == abi_data.name)
        .map(|&x| x.abi)
}

pub fn all_names() -> Vec<&'static str> {
    AbiDatas.iter().map(|d| d.name).collect()
}

impl Abi {
    #[inline]
    pub fn index(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub fn data(&self) -> &'static AbiData {
        &AbiDatas[self.index()]
    }

    pub fn name(&self) -> &'static str {
        self.data().name
    }
}

impl fmt::Display for Abi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.name())
    }
}

#[allow(non_snake_case)]
#[test]
fn lookup_Rust() {
    let abi = lookup("Rust");
    assert!(abi.is_some() && abi.unwrap().data().name == "Rust");
}

#[test]
fn lookup_cdecl() {
    let abi = lookup("cdecl");
    assert!(abi.is_some() && abi.unwrap().data().name == "cdecl");
}

#[test]
fn lookup_baz() {
    let abi = lookup("baz");
    assert!(abi.is_none());
}

#[test]
fn indices_are_correct() {
    for (i, abi_data) in AbiDatas.iter().enumerate() {
        assert_eq!(i, abi_data.abi.index());
    }
}
