#[macro_use]
extern crate bitflags;

use std::fmt;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]
pub mod smi {
    include!("bindings.rs");
}

bitflags! {
    pub struct SmiNodeKind: u32 {
        const UNKNOWN = 0x0000;
        const NODE = 0x0001;
        const SCALAR = 0x0002;
        const TABLE = 0x0004;
        const ROW = 0x0008;
        const COLUMN = 0x0010;
        const NOTIFICATION = 0x0020;
        const GROUP = 0x0040;
        const COMPLIANCE = 0x0080;
        const CAPABILITIES = 0x0100;
        const ANY = 0xFFFF;
    }
}

impl fmt::Display for SmiNodeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut kinds: Vec<&str> = Vec::new();
        if self.contains(SmiNodeKind::NODE) {
            kinds.push("NODE");
        }
        if self.contains(SmiNodeKind::SCALAR) {
            kinds.push("SCALAR");
        }
        if self.contains(SmiNodeKind::TABLE) {
            kinds.push("TABLE");
        }
        if self.contains(SmiNodeKind::ROW) {
            kinds.push("ROW");
        }
        if self.contains(SmiNodeKind::COLUMN) {
            kinds.push("COL");
        }
        if self.contains(SmiNodeKind::NOTIFICATION) {
            kinds.push("NOTIFICATION");
        }
        if self.contains(SmiNodeKind::GROUP) {
            kinds.push("GROUP");
        }
        if self.contains(SmiNodeKind::COMPLIANCE) {
            kinds.push("COMPLIANCE");
        }
        if self.contains(SmiNodeKind::CAPABILITIES) {
            kinds.push("CAPABILITIES");
        }

        if self.is_empty() {
            write!(f, "UNKNOWN")
        } else {
            write!(f, "{}", kinds.join("|"))
        }
    }
}

impl From<u32> for SmiNodeKind {
    fn from(n: u32) -> Self {
        SmiNodeKind::from_bits_truncate(n)
    }
}

bitflags! {
    pub struct RenderFlags: i32 {
        const NUMERIC = 0x0001;
        const NAME = 0x0002;
        const QUALIFIED = 0x0004;
        const FORMATTED = 0x0008;
        const PRINTABLE = 0x0010;
        const UNKNOWN   = 0x0020;
        const ALL       = 0x00FF;
    }
}

mod context;
pub use context::*;

mod node;
pub use node::*;