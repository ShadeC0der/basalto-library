pub fn run(args: &[&str]) {
    /* Resumen de run(args)
     * Verifica que se pasó una ruta como argumento
     * Si el archivo ya existe, avisa y termina
     * Crea los directorios del path si no existen
     * Crea el archivo y confirma al usuario
     */

    if args.is_empty() {
        println!("Uso: basalto add <ruta>");
        return;
    }

    let entry = args[0];
    let home = dirs::home_dir().unwrap();
    let file_path = format!(
        "{}/.basalto/cache/library/{}",
        home.to_str().unwrap(),
        entry
    );
    let file = std::path::Path::new(&file_path);

    if file.exists() {
        println!("'{}' ya existe.", entry);
        return;
    }

    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::File::create(&file_path).unwrap();
    println!("'{}' creado.", entry);
}

