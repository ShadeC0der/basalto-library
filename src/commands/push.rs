use crate::config;

pub fn run(_args: &[&str]) {
    /* Resumen de run(args)
     * Lee la URL y branch de la biblioteca desde config.toml
     * Verifica que la URL esté configurada
     * Verifica que el directorio de la biblioteca sea un repo git
     * Hace git add -A, commit con timestamp, y git push
     */

    let lib_conf = config::read_library();

    if lib_conf.url.is_empty() {
        println!("No hay URL configurada en [library] url de ~/.basalto/config.toml");
        return;
    }

    let home = dirs::home_dir().unwrap();
    let lib_dir = crate::index::lib_path();
    let git_dir = format!("{}/.git", lib_dir);

    if !std::path::Path::new(&git_dir).exists() {
        println!("La biblioteca no es un repositorio git.");
        println!("Inicializa el repo con:");
        println!("  cd {} && git init && git remote add origin {}", lib_dir, lib_conf.url);
        return;
    }

    let timestamp = std::process::Command::new("date")
        .args(["+%Y-%m-%d %H:%M:%S"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "sin fecha".to_string());

    let timestamp = timestamp.trim();

    std::process::Command::new("git")
        .args(["add", "-A"])
        .current_dir(&lib_dir)
        .status()
        .unwrap();

    let commit_msg = format!("basalto: actualizacion {}", timestamp);

    let status = std::process::Command::new("git")
        .args(["commit", "-m", &commit_msg])
        .current_dir(&lib_dir)
        .status()
        .unwrap();

    if !status.success() {
        println!("No hay cambios para publicar.");
        return;
    }

    println!("Publicando en {}...", lib_conf.url);

    std::process::Command::new("git")
        .args(["push", "origin", &lib_conf.branch])
        .current_dir(&lib_dir)
        .status()
        .unwrap();

    println!("Biblioteca publicada.");
}
