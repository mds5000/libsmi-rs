use crate::smi;
use crate::{SmiNode, SmiNodeKind, RenderFlags};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::fmt;
use std::slice;

pub type Error = i32;
pub struct SmiContext {}

impl SmiContext {
    pub fn init() -> Result<SmiContext, Error> {
        let ret = unsafe { smi::smiInit(std::ptr::null()) };

        if ret == 0 {
            return Ok(SmiContext {});
        }

        Err(ret)
    }

    pub fn set_search_path(&mut self, path: &str) -> Result<(), Error> {
        let cstr = CString::new(path).unwrap();
        unsafe {
            let res = smi::smiSetPath(cstr.as_ptr());
            if res != 0 {
                return Err(res);
            }
        }
        Ok(())
    }

    pub fn get_search_path(&self) -> &str {
        unsafe {
            let res = smi::smiGetPath();
            CStr::from_ptr(res).to_str().unwrap()
            // FIXME: Memory Leak! This pointer is supposed to be freed by the caller.
        }
    }

    pub fn load_module(&mut self, path: &str) -> Result<(), Error> {
        let cstr = CString::new(path).unwrap();
        unsafe {
            let res = smi::smiLoadModule(cstr.as_ptr());
            if res.is_null() {
                return Err(-1);
            }
        }
        Ok(())
    }

    pub fn lookup_node<'a>(&self, node: &str) -> Option<SmiNode<'a>> {
        let null = std::ptr::null_mut();
        let cstr = CString::new(node).unwrap();
        unsafe {
            let ptr = smi::smiGetNode(null, cstr.as_ptr());
            if ptr.is_null() {
                return None;
            }
            Some(SmiNode::from_ptr(ptr))
        }
    }

    pub fn lookup_oid(&self, oid: &[u32]) -> Option<SmiNode> {
        unsafe {
            // We are sure that smiGetNodeByOID does not modify its arguments, though
            // the C API does not explicitly call out a const ptr. So we need to manually
            // coerce the const ptr into a mutable one.
            let oid_ptr = std::mem::transmute::<*const u32, *mut u32>(oid.as_ptr());
            let ptr = smi::smiGetNodeByOID(oid.len() as u32, oid_ptr);
            if ptr.is_null() {
                return None;
            }
            Some(SmiNode::from_ptr(ptr))
        }
    }
}

impl Drop for SmiContext {
    fn drop(&mut self) {
        unsafe {
            smi::smiExit();
        }
    }
}

