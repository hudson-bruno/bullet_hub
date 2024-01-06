use godot::prelude::*;
struct MyExtension;

pub(crate) mod bh_server;
pub(crate) mod module_bindings;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
