use basalto_shared::BasaltoPlugin;
mod commands;

struct BasaltoCli;

impl BasaltoPlugin for BasaltoCli {
    fn name(&self) -> &str {
        "basalto_cli"
    }

    fn plugin_commands(&self) -> &[&str] {
        &["help", "version"]
    }

    fn on_load(&self) {}

    fn execute_command(&self, command: &str, args: &[&str]) {
        match command {
            "help" => commands::help(args),
            "version" => commands::version(args),
            _ => {}
        }
    }
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn _basalto_create_plugin() -> *mut dyn BasaltoPlugin {
    Box::into_raw(Box::new(BasaltoCli))
}
