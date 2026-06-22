# basalto-library

Plugin oficial de Basalto para gestionar una biblioteca personal de archivos. Permite crear, visualizar, editar y publicar archivos desde la terminal.

## Qué hace

Mantiene una biblioteca de archivos en `~/.basalto/cache/library/`. Los archivos se organizan en carpetas libremente y se sincronizan con un repositorio git remoto.

## Comandos

| Comando | Descripción |
|---|---|
| `basalto add <ruta>` | Crea un archivo en la biblioteca |
| `basalto show` | Muestra el árbol completo de la biblioteca |
| `basalto show <ruta>` | Muestra el contenido de un archivo |
| `basalto edit <ruta>` | Abre el archivo con el editor configurado |
| `basalto push` | Publica los cambios al repositorio remoto |

## Instalación

Crea el archivo `~/.basalto/plugins/basalto-library.toml`:

```toml
source = "git@github.com:ShadeC0der/basalto-library.git"
branch = "main"
enabled = true
```

En el próximo arranque, basalto-core clona y compila el plugin automáticamente. También se puede instalar manualmente con:

```
basalto update
```

## Configuración

En `~/.basalto/config.toml`:

```toml
[library]
url = "git@github.com:usuario/mi-biblioteca.git"
branch = "main"

[editors]
available = ["nvim", "vim", "nano"]
```

- `url` — repositorio remoto donde se publican los archivos con `basalto push`
- `available` — lista de editores para `basalto edit`. Si hay más de uno, muestra un menú interactivo para elegir

## Ejemplo de uso

```bash
# Crear un archivo nuevo
basalto add notas/rust.md

# Editarlo con el editor configurado
basalto edit notas/rust.md

# Ver el árbol de la biblioteca
basalto show

# Ver el contenido de un archivo
basalto show notas/rust.md

# Publicar los cambios
basalto push
```

## Dependencias

- [basalto-shared](https://github.com/ShadeC0der/basalto-shared) `v1.2.0` — trait `BasaltoPlugin`
- [dialoguer](https://crates.io/crates/dialoguer) — menú interactivo de selección de editor
