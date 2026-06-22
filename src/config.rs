use serde::Deserialize;

#[derive(Deserialize)]
pub struct LibraryConfig {
    pub url: String,
    pub branch: String,
}

#[derive(Deserialize)]
pub struct EditorsConfig {
    pub available: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    library: LibraryConfig,
    editors: EditorsConfig,
}

pub fn read_library() -> LibraryConfig {
    /* Resumen de read_library()
     * Lee config.toml y retorna la sección [library]
     */
    read_config().library
}

pub fn read_editors() -> EditorsConfig {
    /* Resumen de read_editors()
     * Lee config.toml y retorna la sección [editors]
     */
    read_config().editors
}

fn read_config() -> Config {
    /* Resumen de read_config()
     * Construye la ruta hacia ~/.basalto/config.toml
     * Lee el archivo y lo deserializa completo
     */
    let home = dirs::home_dir().unwrap();
    let path = format!("{}/.basalto/config.toml", home.to_str().unwrap());
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("No se encontró config.toml en {}", path));

    toml::from_str(&text).unwrap_or_else(|_| panic!("Error al leer config.toml"))
}
