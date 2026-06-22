use crate::index;
use console::style;
use dialoguer::{Confirm, Select};

pub fn run(args: &[&str]) {
    let home = dirs::home_dir().unwrap();
    let lib_path = crate::index::lib_path();

    let target = if args.is_empty() {
        let entradas = listar_entradas(&lib_path);

        if entradas.is_empty() {
            println!("La biblioteca esta vacia. Usa {} para agregar archivos.", style("basalto add").bold());
            return;
        }

        let labels: Vec<String> = entradas
            .iter()
            .map(|f| f.trim_start_matches(&format!("{}/", lib_path)).to_string())
            .collect();

        let seleccion = Select::new()
            .with_prompt("Eliminar")
            .items(&labels)
            .interact()
            .unwrap();

        entradas[seleccion].clone()
    } else {
        let path = format!("{}/{}", lib_path, args[0]);
        if !std::path::Path::new(&path).exists() {
            println!("'{}' no existe.", args[0]);
            return;
        }
        path
    };

    let label = target.trim_start_matches(&format!("{}/", lib_path));
    let is_dir = std::path::Path::new(&target).is_dir();

    let confirmacion = Confirm::new()
        .with_prompt(format!(
            "Eliminar {} '{}'?",
            if is_dir { "carpeta" } else { "archivo" },
            style(label).bold()
        ))
        .default(false)
        .interact()
        .unwrap();

    if !confirmacion {
        println!("Cancelado.");
        return;
    }

    if is_dir {
        std::fs::remove_dir_all(&target).unwrap();
        println!("{} '{}'", style("Carpeta eliminada:").red(), style(label).bold());
    } else {
        std::fs::remove_file(&target).unwrap();
        index::eliminar(label);
        println!("{} '{}'", style("Archivo eliminado:").red(), style(label).bold());
    }
}

fn listar_entradas(dir: &str) -> Vec<String> {
    let mut entradas = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        let mut entries: Vec<_> = entries.flatten().collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();
            entradas.push(path_str.clone());
            if path.is_dir() {
                entradas.extend(listar_entradas(&path_str));
            }
        }
    }

    entradas
}
