use dyn_load::{Plugin, System};
use libloading::{Error, Library};

type StartSig = unsafe fn(&mut dyn System) -> Box<dyn Plugin>;
// type StopSig = unsafe fn(&mut dyn System);
type DestroySig = unsafe fn(Box<dyn Plugin>);

pub struct TestSystem {
    plugins: Vec<InternalPlugin>,
    num: u64,
}

impl TestSystem {
    fn load_plugin(&mut self, name: &str) -> Result<(), Error> {
        unsafe {
            let lib = Library::new(name)?;
            let start: StartSig = *lib.get(b"start\0")?;
            let destroy: DestroySig = *lib.get(b"destroy\0")?;
            let inner = start(self);
            self.plugins.push(InternalPlugin {
                lib,
                inner,
                destroy,
            });
        }
        Ok(())
    }
}

impl Default for TestSystem {
    fn default() -> Self {
        Self {
            plugins: Vec::with_capacity(0),
            num: 0,
        }
    }
}

impl System for TestSystem {
    fn num(&self) -> u64 {
        self.num
    }
}

impl Drop for TestSystem {
    fn drop(&mut self) {
        let plugins = std::mem::replace(&mut self.plugins, Vec::with_capacity(0));
        for InternalPlugin {
            lib,
            inner,
            destroy,
        } in plugins
        {
            unsafe { destroy(inner) };
            drop(lib);
        }
    }
}

pub struct InternalPlugin {
    lib: Library,
    inner: Box<dyn Plugin>,
    destroy: DestroySig,
}

fn main() {
    let mut system = TestSystem::default();
    system.load_plugin("./test.plugin").unwrap();
}
