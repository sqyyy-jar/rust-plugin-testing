use dyn_load_api::System;
use libloading::{Error, Library};

type OnLoadSig = unsafe fn(&dyn System);

pub struct TestSystem {
    num: u64,
}

impl System for TestSystem {
    fn get_num(&self) -> u64 {
        self.num
    }
}

fn main() {
    let system = TestSystem { num: 42 };
    load_plugin(&system, "./test.plugin").unwrap();
}

fn load_plugin(system: &TestSystem, name: &str) -> Result<(), Error> {
    unsafe {
        let lib = Library::new(name)?;
        let on_load: OnLoadSig = *lib.get(b"on_load\0")?;
        on_load(system);
    }
    Ok(())
}
