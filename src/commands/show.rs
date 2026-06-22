pub fn run(args: &[&str]) {
    /* Resumen de run(args)
     * Si no hay argumentos muestra el arbol completo de la biblioteca
     * Si hay una ruta verifica que el archivo existe y muestra su contenido
     */

    let home = dirs::home_dir().unwrap();
    let lib_path = format!("{}/.basalto/cache/library", home.to_str().unwrap());

    if args.is_empty() {
        println!("biblioteca");
        show_tree(&lib_path, 0);
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

fn show_tree(path: &str, depth: usize) {
    /* Resumen de show_tree(path, depth)
     * Lee el contenido del directorio actual
     * Ordena las entradas por nombre
     * Imprime cada entrada con indentacion segun la profundidad
     * Si la entrada es un directorio se llama recursivamente
     */

    let mut entries: Vec<_> = std::fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let is_dir = entry.file_type().unwrap().is_dir();
        let indent = "│   ".repeat(depth);
        let display = if is_dir {
            format!("{}/", name)
        } else {
            name.to_string()
        };
        println!("{}├── {}", indent, display);

        if is_dir {
            show_tree(&entry.path().to_string_lossy(), depth + 1);
        }
    }
}
