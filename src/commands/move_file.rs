use crate::index;
use console::style;
use dialoguer::{Input, Select};

pub fn run(args: &[&str]) {
    let home = dirs::home_dir().unwrap();
    let lib_path = crate::index::lib_path();

    let origen = if args.is_empty() {
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
            .with_prompt("Mover")
            .items(&labels)
            .interact()
            .unwrap();

        archivos[seleccion].clone()
    } else {
        let path = format!("{}/{}", lib_path, args[0]);
        if !std::path::Path::new(&path).exists() {
            println!("'{}' no existe. Usa {} para crearlo.", args[0], style("basalto add").bold());
            return;
        }
        path
    };

    let origen_label = origen.trim_start_matches(&format!("{}/", lib_path)).to_string();

    let destino_relativo: String = if args.len() >= 2 {
        args[1].to_string()
    } else {
        Input::new()
            .with_prompt("Nueva ruta")
            .with_initial_text(&origen_label)
            .interact_text()
            .unwrap()
    };

    if destino_relativo.trim().is_empty() {
        println!("{}", style("La ruta de destino no puede estar vacia.").red());
        return;
    }

    let destino = format!("{}/{}", lib_path, destino_relativo);

    if destino == origen {
        println!("El origen y el destino son iguales.");
        return;
    }

    if std::path::Path::new(&destino).exists() {
        println!("'{}' ya existe.", destino_relativo);
        return;
    }

    if let Some(parent) = std::path::Path::new(&destino).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    std::fs::rename(&origen, &destino).unwrap();
    index::renombrar(&origen_label, &destino_relativo);
    println!(
        "{} '{}' → '{}'",
        style("Movido:").green(),
        style(&origen_label).bold(),
        style(&destino_relativo).bold()
    );
}

fn listar_archivos(dir: &str) -> Vec<String> {
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
