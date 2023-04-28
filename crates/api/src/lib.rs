use std::{ffi::{c_char, CStr}, fmt::Debug};

#[derive(Clone)]
#[repr(C)]
pub struct Metadata {
    pub name: *const c_char,
    pub cli: bool,
}

impl Debug for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Metadata")
            .field("name", &unsafe { CStr::from_ptr(self.name) })
            .field("cli", &self.cli)
            .finish()
    }
}

unsafe impl std::marker::Sync for Metadata {}

pub struct Plugin {}
