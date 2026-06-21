use basalto_shared::BasaltoPlugin;
mod commands;
mod config;

struct BasaltoLibrary;

impl BasaltoPlugin for BasaltoLibrary {
    fn name(&self) -> &str {
        "basalto_library"
    }

    fn plugin_commands(&self) -> &[&str] {
        &["show", "add"]
    }

    fn on_load(&self) {}

    fn execute_command(&self, command: &str, args: &[&str]) {
        match command {
            "show" => commands::show(args),
            "add" => commands::add(args),
            _ => {}
        }
    }
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn _basalto_create_plugin() -> *mut dyn BasaltoPlugin {
    Box::into_raw(Box::new(BasaltoLibrary))
}
