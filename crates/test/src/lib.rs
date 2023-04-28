use std::ffi::c_char;

use dyn_load::Metadata;

static PLUGIN_METADATA: Metadata = Metadata {
    name: "my-plugin".as_ptr() as *const c_char,
    cli: false,
};

#[no_mangle]
pub fn get_metadata() -> &'static Metadata {
    &PLUGIN_METADATA
}
