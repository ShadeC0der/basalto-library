use console::style;
use dialoguer::Input;

pub fn run(args: &[&str]) {
    /* Resumen de run(args)
     * Sin args: pide la ruta con un prompt interactivo
     * Si la ruta termina en '/': crea una carpeta
     * Si no: crea un archivo vacío con los directorios necesarios
     * Avisa si ya existe
     */

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

    let entrada = entrada.trim();

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
        println!("{} '{}'", style("Carpeta creada:").green(), style(entrada).bold());
    } else {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::File::create(&full_path).unwrap();
        println!("{} '{}'", style("Archivo creado:").green(), style(entrada).bold());
    }
}
