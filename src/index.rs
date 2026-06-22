use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct EntradaMeta {
    pub description: String,
    pub tags: Vec<String>,
    pub usos: u32,
    pub agregado: String,
}

#[derive(Serialize, Deserialize, Default)]
struct LibraryIndex {
    #[serde(default)]
    files: HashMap<String, EntradaMeta>,
}

pub fn agregar(ruta: &str, description: &str, tags: Vec<String>) {
    let mut index = leer();
    index.files.insert(
        ruta.to_string(),
        EntradaMeta {
            description: description.to_string(),
            tags,
            usos: 0,
            agregado: fecha_hoy(),
        },
    );
    guardar(&index);
}

pub fn incrementar_usos(ruta: &str) {
    let mut index = leer();
    if let Some(entrada) = index.files.get_mut(ruta) {
        entrada.usos += 1;
    }
    guardar(&index);
}

pub fn eliminar(ruta: &str) {
    let mut index = leer();
    index.files.remove(ruta);
    guardar(&index);
}

pub fn renombrar(origen: &str, destino: &str) {
    let mut index = leer();
    if let Some(entrada) = index.files.remove(origen) {
        index.files.insert(destino.to_string(), entrada);
    }
    guardar(&index);
}

pub fn obtener(ruta: &str) -> Option<EntradaMeta> {
    leer().files.get(ruta).cloned()
}

pub fn todos() -> HashMap<String, EntradaMeta> {
    leer().files
}

fn leer() -> LibraryIndex {
    std::fs::read_to_string(ruta_index())
        .ok()
        .and_then(|c| toml::from_str(&c).ok())
        .unwrap_or_default()
}

fn guardar(index: &LibraryIndex) {
    std::fs::write(ruta_index(), toml::to_string(index).unwrap()).unwrap();
}

fn ruta_index() -> String {
    let home = dirs::home_dir().unwrap();
    format!("{}/.basalto/library.index.toml", home.to_str().unwrap())
}

fn fecha_hoy() -> String {
    std::process::Command::new("date")
        .arg("+%Y-%m-%d")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string()
}
