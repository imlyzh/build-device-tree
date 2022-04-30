use std::{ffi::{CString, CStr}, cell::RefCell};

/*
    Copy from serde-device-tree
    Copyright (c) luojia
*/

pub const DEVICE_TREE_MAGIC: u32 = 0xD00DFEED;
pub const FDT_BEGIN_NODE: u32 = 0x1;
pub const FDT_END_NODE: u32 = 0x2;
pub const FDT_PROP: u32 = 0x3;
pub const FDT_NOP: u32 = 0x4;
pub const FDT_END: u32 = 0x9;
pub const SUPPORTED_VERSION: u32 = 17;

#[derive(Debug, Clone)]
#[repr(C)]
#[repr(packed)]
pub struct Header {
    pub magic: u32,
    pub total_size: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}

pub const HEADER_LEN: u32 = core::mem::size_of::<Header>() as u32;

#[inline]
pub fn align_up_u32(val: usize) -> usize {
    val + (4 - (val % 4)) % 4
}

///////////////////////////////////////////////////////////////////



#[derive(Debug, Clone)]
pub struct DT {
    // rmems: FdtReservedMem,
    pub nodes: Vec<BeginNode>,
}

/*
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FdtReservedMem {
    pub address: u64,
    pub size: u64,
}
 */

#[derive(Debug, Clone)]
pub struct BeginNode {
    pub name: CString,
    pub children: Childrens,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub value: Vec<u8>,
    pub name: CString,
    pub children: Childrens,
}

#[derive(Debug, Clone)]
pub struct Childrens(pub Vec<TreeNode>);


#[derive(Debug, Clone)]
pub struct CStringBuffer(pub RefCell<Vec<u8>>);

impl CStringBuffer {
    pub fn new() -> Self {
        CStringBuffer(RefCell::new(Vec::new()))
    }

    pub fn intern(&self, i: &CStr) -> usize {
        let start_pos = {self.0.borrow().len()};
        self.0.borrow_mut().extend_from_slice(i.to_bytes());
        start_pos
    }
}