

use std::{ops::AddAssign, ffi::CString};

use crate::datastructs::*;


impl DT {
    fn dump(&self, buf: &mut Vec<u8>, mut start_pos: usize, boot_cpuid_phys: u32) -> Option<()> {
        let cur_start = start_pos + HEADER_LEN as usize;
        let mut cur = cur_start;
        let cstr_buf = CStringBuffer::new();

        let off_dt_struct = cur;

        for i in &self.nodes {
            i.dump(buf, &mut cur, &cstr_buf)?;
        }


        let new_cur = align_up_u32(cur);
        debug_assert_eq!(new_cur, cur);
        cur = new_cur;


        let off_dt_strings = cur;

        let end_of_buf = off_dt_strings+cstr_buf.0.borrow().len();
        buf[off_dt_strings..end_of_buf]
            .copy_from_slice(cstr_buf.0.borrow().as_slice());
        cur = end_of_buf;


        let new_cur = align_up_u32(cur);
        debug_assert_eq!(new_cur, cur);
        cur = new_cur;


        write_u32(buf, &mut cur, FDT_END);

        let h = Header {
            magic: DEVICE_TREE_MAGIC.to_be(),
            total_size: ((cur - start_pos) as u32).to_be(),
            off_dt_struct: (off_dt_struct as u32).to_be(),
            off_dt_strings: (off_dt_strings as u32).to_be(),
            off_mem_rsvmap: (0u32).to_be(),
            version: SUPPORTED_VERSION.to_be(),
            last_comp_version: SUPPORTED_VERSION.to_be(),
            boot_cpuid_phys: (boot_cpuid_phys).to_be(),
            size_dt_strings: 0u32.to_be(),
            size_dt_struct: 0u32.to_be(),
        };
        h.dump(buf, &mut start_pos)?;

        Some(())
    }
}

impl Header {
    pub fn dump(&self, buf: &mut Vec<u8>, start_pos: &mut usize) -> Option<()> {
        buf[*start_pos..(*start_pos + HEADER_LEN as usize)].copy_from_slice(
            unsafe {
                core::slice::from_raw_parts(
                    self as *const Header as *const u8,
                    HEADER_LEN as usize,
                )
            }
        );
        Some(())
    }
}

impl BeginNode {
    pub fn dump(&self, buf: &mut Vec<u8>, start_pos: &mut usize, string_buf: &CStringBuffer) -> Option<()> {
        write_u32(buf, start_pos, FDT_BEGIN_NODE);

        write_string_aligned4(buf, start_pos, &self.name);

        self.children.dump(buf, start_pos, string_buf)?;

        write_u32(buf, start_pos, FDT_END_NODE);

        Some(())
    }
}

impl TreeNode {
    pub fn dump(&self, buf: &mut Vec<u8>, start_pos: &mut usize, string_buf: &CStringBuffer) -> Option<()> {
        write_u32(buf, start_pos, FDT_PROP);

        write_u32(buf, start_pos, self.value.len() as u32);

        let str_offset = string_buf.intern(self.name.as_c_str());
        write_u32(buf, start_pos, str_offset as u32);

        write_slice_aligned4(buf, start_pos, &self.value);

        self.children.dump(buf, start_pos, string_buf)?;

        write_u32(buf, start_pos, FDT_END_NODE);
        Some(())
    }
}

impl Childrens {
    pub fn dump(&self, buf: &mut Vec<u8>, start_pos: &mut usize, string_buf: &CStringBuffer) -> Option<()> {
        for node in self.0.iter() {
            node.dump(buf, start_pos, string_buf)?;
        }
        Some(())
    }
}

/*
#[inline]
fn write_string_aligned4(buf: &mut Vec<u8>, start_pos: &mut usize, str: &CString) {
    buf[*start_pos..*start_pos+str.as_bytes().len()]
            .copy_from_slice(str.as_bytes());
    *start_pos += str.as_bytes().len();
    *start_pos = align_up_u32(*start_pos);
}
 */

#[inline]
fn write_string_aligned4(buf: &mut Vec<u8>, start_pos: &mut usize, str: &CString) {
    write_slice_aligned4(buf, start_pos, str.as_bytes())
}

#[inline]
fn write_slice_aligned4(buf: &mut Vec<u8>, start_pos: &mut usize, slice: &[u8]) {
    buf[*start_pos..*start_pos+slice.len()]
            .copy_from_slice(slice);
    *start_pos += slice.len();
    *start_pos = align_up_u32(*start_pos);
}

#[inline]
fn write_u32(buf: &mut Vec<u8>, start_pos: &mut usize, val: u32) {
    let val = val.to_be_bytes();
    buf.insert(*start_pos, val[0]);
    start_pos.add_assign(1);
    buf.insert(*start_pos, val[1]);
    start_pos.add_assign(1);
    buf.insert(*start_pos, val[2]);
    start_pos.add_assign(1);
    buf.insert(*start_pos, val[3]);
    start_pos.add_assign(1);
}