pub fn run(args: &[&str]) {
    /* Resumen de run(args)
     * Si no hay argumentos muestra el arbol completo de la biblioteca
     * Si hay una ruta verifica que el archivo existe y muestra su contenido
     */

    let home = dirs::home_dir().unwrap();
    let lib_path = format!("{}/.basalto/cache/library", home.to_str().unwrap());

    if args.is_empty() {
        println!("biblioteca");
        show_tree(&lib_path, "");
        return;
    }

    let entry = args[0];
    let file_path = format!("{}/{}", lib_path, entry);

    if !std::path::Path::new(&file_path).exists() {
        println!("'{}' no existe.", entry);
        return;
    }

    let content = std::fs::read_to_string(&file_path).unwrap();
    println!("{}", content);
}

fn show_tree(path: &str, prefix: &str) {
    /* Resumen de show_tree(path, prefix)
     * Lee el contenido del directorio actual y ordena por nombre
     * El prefix acumula los caracteres de continuacion de niveles superiores
     * Usa └── para el ultimo elemento y ├── para los demas
     * Pasa "    " como continuacion si el padre fue ultimo, "│   " si no lo fue
     */

    let mut entries: Vec<_> = std::fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    entries.sort_by_key(|e| e.file_name());

    let total = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let is_dir = entry.file_type().unwrap().is_dir();
        let is_last = i + 1 == total;
        let connector = if is_last { "└──" } else { "├──" };
        let display = if is_dir { format!("{}/", name) } else { name.to_string() };

        println!("{}{} {}", prefix, connector, display);

        if is_dir {
            let child_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            show_tree(&entry.path().to_string_lossy(), &child_prefix);
        }
    }
}
