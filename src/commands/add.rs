use crate::index;
use console::style;
use dialoguer::Input;

pub fn run(args: &[&str]) {
    let home = dirs::home_dir().unwrap();
    let lib_path = format!("{}/.basalto/cache/library", home.to_str().unwrap());

    let entrada: String = if args.is_empty() {
        Input::new()
            .with_prompt("Ruta")
            .interact_text()
            .unwrap()
    } else {
        args[0].to_string()
    };

    let entrada = entrada.trim().to_string();

    if entrada.is_empty() {
        println!("{}", style("La ruta no puede estar vacia.").red());
        return;
    }

    let full_path = format!("{}/{}", lib_path, entrada);
    let path = std::path::Path::new(&full_path);

    if path.exists() {
        println!("'{}' ya existe.", style(entrada).yellow());
        return;
    }

    if entrada.ends_with('/') {
        std::fs::create_dir_all(&full_path).unwrap();
        println!("{} '{}'", style("Carpeta creada:").green(), style(&entrada).bold());
    } else {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::File::create(&full_path).unwrap();
        println!("{} '{}'", style("Archivo creado:").green(), style(&entrada).bold());

        let descripcion: String = Input::new()
            .with_prompt("Descripcion (Enter para omitir)")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        let tags_input: String = Input::new()
            .with_prompt("Tags (ej: rust,async — Enter para omitir)")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        let tags: Vec<String> = tags_input
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();

        index::agregar(&entrada, descripcion.trim(), tags);
    }
}
