pub fn run(args: &[&str]) {
    /* Resumen de run(args)
     * Verifica que se pasó una ruta como argumento
     * Si el archivo no existe, avisa y termina
     * Lee el contenido del archivo y lo imprime
     */

    if args.is_empty() {
        println!("Uso: basalto show <ruta>");
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
        println!("'{}' no existe.", entry);
        return;
    }

    let content = std::fs::read_to_string(&file_path).unwrap();
    println!("{}", content);
}

