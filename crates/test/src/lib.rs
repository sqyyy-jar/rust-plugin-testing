use dyn_load_api::System;

#[no_mangle]
fn on_load(system: &dyn System) {
    println!("{}", system.get_num());
}
