use crate::smi;
use crate::{SmiNodeKind, RenderFlags};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::fmt;

#[derive(FromPrimitive, Debug, Copy, Clone, PartialEq)]
pub enum SmiStatus {
    Unknown = 0,
    Current,
    Deprecated,
    Mandatory,
    Optional,
    Obsolete
}

#[derive(FromPrimitive, Debug, Copy, Clone, PartialEq)]
pub enum SmiAccess {
    Unknown = 0,
    NotImplemented,
    NotAccessible,
    Notify,
    ReadOnly,
    ReadWrite,
    Install,
    InstallNotify,
    ReportOnly,
    EventOnly,
}

pub struct TextualConvention<'a> {
    this: *mut smi::SmiType,
    pub name: &'a str,
    pub basetype: smi::SmiBasetype,
    pub format: &'a str,
    pub units: &'a str,
    pub status: SmiStatus,
    pub description: &'a str,
    pub reference: &'a str,
}
impl<'a> TextualConvention<'a> {
    unsafe fn from_ptr(this: *mut smi::SmiType) -> TextualConvention<'a> {
        let t = *this;
        let name = cstr_from_ptr(t.name);
        let basetype = t.basetype;
        let status = FromPrimitive::from_u32(t.status).unwrap_or(SmiStatus::Unknown);
        let format = cstr_from_ptr(t.format);
        let units = cstr_from_ptr(t.units);
        let description = cstr_from_ptr(t.description);
        let reference = cstr_from_ptr(t.reference);

        TextualConvention {
            this,
            name,
            basetype,
            format,
            units,
            status,
            description,
            reference
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct SmiNode<'a> {
    this: *mut smi::SmiNode,
    pub name: &'a str,
    pub oid: &'a [u32],
    pub access: SmiAccess,
    pub status: SmiStatus,
    pub format: &'a str,
    pub units: &'a str,
    pub description: &'a str,
    pub reference: &'a str,
    pub kind: SmiNodeKind,
}

pub struct SmiNodeIterator<'a> {
    first: bool,
    node: *mut smi::SmiNode,
    marker: std::marker::PhantomData<&'a SmiNode<'a>>,
}

unsafe fn cstr_from_ptr<'a>(s: *mut std::os::raw::c_char) -> &'a str {
    if s.is_null() {
        return "";
    }

    CStr::from_ptr(s).to_str().unwrap_or("<invalid UTF8 string>")
}

impl<'a> SmiNode<'a> {
    unsafe fn from_ptr(this: *mut smi::SmiNode) -> SmiNode<'a> {
        let t = *this;
        let name = cstr_from_ptr(t.name);
        let oid: &[u32] = slice::from_raw_parts(t.oid, t.oidlen.try_into().unwrap());
        let format = cstr_from_ptr(t.format);
        let access = FromPrimitive::from_u32(t.access).unwrap_or(SmiAccess::Unknown);
        let status = FromPrimitive::from_u32(t.status).unwrap_or(SmiStatus::Unknown);
        let units = cstr_from_ptr(t.units);
        let description = cstr_from_ptr(t.description);
        let reference = cstr_from_ptr(t.reference);
        let kind = SmiNodeKind::from(t.nodekind);

        SmiNode {
            this,
            name,
            oid,
            access,
            status,
            format,
            units,
            description,
            reference,
            kind,
        }
    }

    pub fn parent(&self) -> Option<SmiNode> {
        unsafe {
        let parent = smi::smiGetParentNode(self.this);
        if parent.is_null() {
            return None;
        }

        Some(SmiNode::from_ptr(parent))
    }
    }

    pub fn children(&self) -> SmiNodeIterator {
        SmiNodeIterator {
            first: true,
            node: self.this,
            marker: std::marker::PhantomData,
        }
    }

    pub fn qualified_name(&self) -> &str {
        unsafe {
            cstr_from_ptr(smi::smiRenderNode(self.this, RenderFlags::QUALIFIED))
        }
    }

    pub fn textual_convention(&self) -> Option<TextualConvention> {
        unsafe {
            let convention = smi::smiGetNodeType(self.this);
            if convention.is_null() {
                return None;
            }

            Some(TextualConvention::from_ptr(convention))
        }
    }
}

impl fmt::Display for SmiNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let oid_str = self
            .oid
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(".");
        write!(f, "Node<{}>/{}: {}, {}", oid_str, self.kind, self.name, self.description)
    }
}

impl<'a> Iterator for SmiNodeIterator<'a> {
    type Item = SmiNode<'a>;

    fn next(&mut self) -> Option<SmiNode<'a>> {
        unsafe {
            let res = match self.first {
                true => {
                    self.first = false;
                    smi::smiGetFirstChildNode(self.node)
                }
                false => smi::smiGetNextChildNode(self.node),
            };
            if res.is_null() {
                return None;
            }

            self.node = res;
            return Some(SmiNode::from_ptr(res));
        }
    }
}