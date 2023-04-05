use dyn_load::{Plugin, System};

pub struct TestPlugin {}

impl Plugin for TestPlugin {}

#[no_mangle]
fn start(system: &dyn System) -> Box<dyn Plugin> {
    println!("{}", system.num());
    Box::new(TestPlugin {})
}

#[no_mangle]
fn destroy(_plugin: Box<dyn Plugin>) {}
