use crate::config;
use crate::index;
use console::style;
use dialoguer::Select;

pub fn run(args: &[&str]) {
    /* Resumen de run(args)
     * Sin args: muestra selector interactivo con todos los archivos de la biblioteca
     * Con ruta: abre directamente ese archivo
     * Si hay mas de un editor, muestra menu de seleccion
     */

    let home = dirs::home_dir().unwrap();
    let lib_path = crate::index::lib_path();

    let file_path = if args.is_empty() {
        let archivos = listar_archivos(&lib_path);

        if archivos.is_empty() {
            println!("La biblioteca esta vacia. Usa {} para agregar archivos.", style("basalto add").bold());
            return;
        }

        let labels: Vec<String> = archivos
            .iter()
            .map(|f| f.trim_start_matches(&format!("{}/", lib_path)).to_string())
            .collect();

        let seleccion = Select::new()
            .with_prompt("Archivo")
            .items(&labels)
            .interact()
            .unwrap();

        archivos[seleccion].clone()
    } else {
        let entry = args[0];
        let path = format!("{}/{}", lib_path, entry);

        if !std::path::Path::new(&path).exists() {
            println!("'{}' no existe. Usa {} para crearlo.", entry, style("basalto add").bold());
            return;
        }

        path
    };

    let editors = config::read_editors();

    if editors.available.is_empty() {
        println!("No hay editores configurados en ~/.basalto/config.toml");
        return;
    }

    let editor = if editors.available.len() == 1 {
        editors.available[0].clone()
    } else {
        let seleccion = Select::new()
            .with_prompt("Editor")
            .items(&editors.available)
            .interact()
            .unwrap();
        editors.available[seleccion].clone()
    };

    let ruta_relativa = file_path.trim_start_matches(&format!("{}/", lib_path)).to_string();
    index::incrementar_usos(&ruta_relativa);

    std::process::Command::new(&editor)
        .arg(&file_path)
        .status()
        .unwrap();
}

fn listar_archivos(dir: &str) -> Vec<String> {
    /* Resumen de listar_archivos(dir)
     * Recorre recursivamente el directorio
     * Retorna solo archivos (no carpetas) ordenados por ruta
     */

    let mut archivos = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        let mut entries: Vec<_> = entries.flatten().collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                archivos.extend(listar_archivos(&path.to_string_lossy()));
            } else {
                archivos.push(path.to_string_lossy().to_string());
            }
        }
    }

    archivos
}
