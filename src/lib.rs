use basalto_shared::{BasaltoPlugin, CommandHelp, FlagHelp};
mod commands;
mod config;

basalto_shared::export_version!();

struct BasaltoLibrary;

impl BasaltoPlugin for BasaltoLibrary {
    fn name(&self) -> &str {
        "basalto_library"
    }

    fn plugin_commands(&self) -> &[&str] {
        &["show", "add", "edit", "push"]
    }

    fn on_load(&self) {}

    fn command_help(&self) -> &'static [CommandHelp] {
        &[
            CommandHelp {
                name: "add [ruta]",
                description: "Crea un archivo o carpeta en la biblioteca",
                flags: &[
                    FlagHelp {
                        name: "<ruta>",
                        description: "Ruta del archivo (termina en / para crear carpeta)",
                    },
                ],
            },
            CommandHelp {
                name: "show",
                description: "Muestra el arbol o el contenido de la biblioteca",
                flags: &[
                    FlagHelp {
                        name: "[ruta]",
                        description: "Archivo especifico a mostrar",
                    },
                ],
            },
            CommandHelp {
                name: "edit <ruta>",
                description: "Abre un archivo con el editor configurado",
                flags: &[],
            },
            CommandHelp {
                name: "push",
                description: "Publica los cambios al repositorio remoto",
                flags: &[],
            },
        ]
    }

    fn execute_command(&self, command: &str, args: &[&str]) {
        match command {
            "add" => commands::add::run(args),
            "show" => commands::show::run(args),
            "edit" => commands::edit::run(args),
            "push" => commands::push::run(args),
            _ => {}
        }
    }
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn _basalto_create_plugin() -> *mut dyn BasaltoPlugin {
    Box::into_raw(Box::new(BasaltoLibrary))
}
