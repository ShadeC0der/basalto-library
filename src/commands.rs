pub fn help(_args: &[&str]) {
    println!("Uso: basalto <comando> [args]");
    println!();
    println!("Comandos disponibles:");
    println!("  help      Muestra esta información");
    println!("  version   Muestra la versión actual");
}

pub fn version(_args: &[&str]) {
    println!("basalto-core v0.5.4");
}
