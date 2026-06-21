use serde::Deserialize;

#[derive(Deserialize)]
pub struct LibraryConfig {
    pub url: String,
    pub branch: String,
}

#[derive(Deserialize)]
struct Config {
    library: LibraryConfig,
}

pub fn read() -> LibraryConfig {
    /* Resumen de read()
     * Construye la ruta hacia ~/.basalto/config.toml
     * Lee el archivo como texto plano
     * Deserializa con serde el bloque [library]
     * Retorna url y branch de la biblioteca personal
     */

    let home = dirs::home_dir().unwrap();
    let path = format!("{}/.basalto/config.toml", home.to_str().unwrap());
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("No se encontró config.toml en {}", path));

    let config: Config =
        toml::from_str(&text).unwrap_or_else(|_| panic!("Error al leer [library] en config.toml"));

    config.library
}
