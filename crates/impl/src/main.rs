use std::ffi::OsStr;

use dyn_load::Metadata;
use libloading::{Error, Library};

type GetMetadata = extern "C" fn() -> &'static Metadata;

fn load_metadata(name: impl AsRef<OsStr>) -> Result<(Library, &'static Metadata), Error> {
    unsafe {
        let lib = Library::new(name)?;
        let get_metadata = *lib.get::<GetMetadata>(b"get_metadata\0")?;
        Ok((lib, get_metadata()))
    }
}

fn main() {
    let (_lib, metadata) = load_metadata("./test.plugin").unwrap();
    println!("{metadata:?}");
}
