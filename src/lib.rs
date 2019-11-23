pub use libc::{c_char, c_int, c_ushort};
pub use std::char::decode_utf16;
pub use std::ptr::null;
pub use std::sync::atomic::{AtomicPtr, Ordering};
pub use std::sync::Arc;

use std::convert::TryInto;

#[macro_export]
macro_rules! qba_slice {
    ($qba: expr, $qba_len: expr) => {
        match (to_usize($qba_len), $qba.is_null()) {
            (Some(len), false) => ::std::slice::from_raw_parts($qba as *const u8, len),
            _ => &[],
        }
    };
}

#[repr(C)]
pub struct COption<T> {
    pub data: T,
    pub some: bool,
}

impl<T> COption<T> {
    pub fn into(self) -> Option<T> {
        if self.some {
            Some(self.data)
        } else {
            None
        }
    }
}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{
    fn from(t: Option<T>) -> COption<T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true,
            }
        } else {
            COption {
                data: T::default(),
                some: false,
            }
        }
    }
}

pub enum QString {}

pub fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe {
        match to_usize(len) {
            Some(len) => ::std::slice::from_raw_parts(str, len),
            None => &[],
        }
    };
    let characters = decode_utf16(utf16.iter().cloned()).map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}

pub enum QByteArray {}

#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1,
}

#[repr(C)]
pub struct QModelIndex {
    pub row: c_int,
    pub internal_id: usize,
}

pub fn to_usize(n: c_int) -> Option<usize> {
    n.try_into().ok()
}

pub fn to_c_int(n: usize) -> c_int {
    // saturate
    n.min(c_int::max_value() as usize) as c_int
}
