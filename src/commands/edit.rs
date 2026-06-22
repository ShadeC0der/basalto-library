use crate::config;
use dialoguer::Select;

pub fn run(args: &[&str]) {
    /* Resumen de run(args)
     * Verifica que se pasó una ruta como argumento
     * Verifica que el archivo existe en la biblioteca
     * Lee los editores disponibles del config
     * Si hay un editor lo abre directamente
     * Si hay más de uno muestra un menú de selección con flechas
     */

    if args.is_empty() {
        println!("Uso: basalto edit <ruta>");
        return;
    }

    let entry = args[0];
    let home = dirs::home_dir().unwrap();
    let file_path = format!(
        "{}/.basalto/cache/library/{}",
        home.to_str().unwrap(),
        entry
    );

    if !std::path::Path::new(&file_path).exists() {
        println!("'{}' no existe. Usa basalto add para crearlo.", entry);
        return;
    }

    let editors = config::read_editors();

    if editors.available.is_empty() {
        println!("No hay editores configurados en ~/.basalto/config.toml");
        return;
    }

    let editor = if editors.available.len() == 1 {
        &editors.available[0]
    } else {
        let seleccion = Select::new()
            .with_prompt("Elegir un editor")
            .items(&editors.available)
            .interact()
            .unwrap();
        &editors.available[seleccion]
    };

    std::process::Command::new(editor)
        .arg(&file_path)
        .status()
        .unwrap();
}
