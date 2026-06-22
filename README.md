# basalto-shared

Contrato de plugins para el ecosistema Basalto. Define el trait `BasaltoPlugin`.

## Uso

Un plugin es un crate de Rust que compila a una librería dinámica `.so` e implementa `BasaltoPlugin`.

### Crear la base

```bash
cargo new basalto-myplugin --lib
cd basalto-myplugin
```

`--lib` genera un crate de librería. Después hay que ajustar el `Cargo.toml` para compilar como librería dinámica y agregar la dependencia de basalto-shared.

**`Cargo.toml`**

```toml
[package]
name = "basalto-myplugin"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
basalto-shared = { git = "ssh://git@github.com/ShadeC0der/basalto-shared.git", tag = "v1.2.0" }
```

**`src/lib.rs`**

```rust
use basalto_shared::BasaltoPlugin;

// Exporta el simbolo de version para que el core valide compatibilidad
basalto_shared::export_version!();

struct MyPlugin;

impl BasaltoPlugin for MyPlugin {
    fn name(&self) -> &str {
        "my_plugin"
    }

    fn plugin_commands(&self) -> &[&str] {
        &["my-command"]
    }

    fn on_load(&self) {}

    fn execute_command(&self, command: &str, args: &[&str]) {
        match command {
            "my-command" => println!("running with args: {:?}", args),
            _ => {}
        }
    }
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn _basalto_create_plugin() -> *mut dyn BasaltoPlugin {
    Box::into_raw(Box::new(MyPlugin))
}
```

`#[unsafe(no_mangle)]` es la sintaxis correcta en edition 2024. `#[allow(improper_ctypes_definitions)]` es necesario porque `dyn BasaltoPlugin` no es un tipo C estándar — es intencional en este sistema de plugins con `libloading`.

---

## export_version!()

Desde `v1.2.0`, todos los plugins deben llamar `basalto_shared::export_version!()` en `lib.rs`. Esto exporta el símbolo `_basalto_shared_version` al `.so`, que el dispatcher lee antes de cargar el plugin.

Si el major version no coincide con el del core, el plugin se omite con un mensaje claro en lugar de crashear:

```
Plugin 'mi-plugin' usa basalto-shared v1.x.x (core usa v2.x.x). Corre: basalto update
```

---

## Soporte para basalto help

El método `command_help()` es opcional — si lo implementas, tus comandos aparecen en `basalto help` con descripción y flags. Si no lo implementas, el plugin sigue funcionando normalmente.

```rust
use basalto_shared::{BasaltoPlugin, CommandHelp, FlagHelp};

impl BasaltoPlugin for MyPlugin {
    // ...

    fn command_help(&self) -> &'static [CommandHelp] {
        &[
            CommandHelp {
                name: "my-command",
                description: "Descripcion del comando",
                flags: &[
                    FlagHelp {
                        name: "--flag",
                        description: "Descripcion del flag",
                    },
                ],
            },
        ]
    }
}
```

---

## Estructura recomendada

`execute_command` es el punto de entrada — la lógica real de cada comando vive en un archivo separado:

**`src/lib.rs`** — enrutador puro, sin lógica

```rust
mod commands;

use basalto_shared::BasaltoPlugin;

basalto_shared::export_version!();

struct MyPlugin;

impl BasaltoPlugin for MyPlugin {
    fn name(&self) -> &str { "my_plugin" }

    fn plugin_commands(&self) -> &[&str] { &["show", "add"] }

    fn on_load(&self) {}

    fn execute_command(&self, command: &str, args: &[&str]) {
        match command {
            "show" => commands::show::run(args),
            "add"  => commands::add::run(args),
            _      => {}
        }
    }
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn _basalto_create_plugin() -> *mut dyn BasaltoPlugin {
    Box::into_raw(Box::new(MyPlugin))
}
```

Para plugins con más de dos o tres comandos, es preferible una carpeta en lugar de un archivo único:

```
src/
├── lib.rs
├── commands/
│   ├── mod.rs      ←  pub mod show; pub mod add;
│   ├── show.rs
│   └── add.rs
```

Cada comando vive aislado — agregar uno nuevo no toca los existentes.

---

## Declarar el plugin en el Core

Para que basalto-core cargue el plugin, el usuario crea un archivo en `~/.basalto/plugins/`:

**`~/.basalto/plugins/mi-plugin.toml`**

```toml
source = "git@github.com:usuario/basalto-myplugin.git"
branch = "main"
enabled = true
```

El Core clona el repo, lo compila y lo carga automáticamente en el próximo arranque. El nombre del archivo `.toml` puede ser cualquiera — el Core lo ignora y usa `name()` del trait para identificar el plugin.

---

## Dependencia

SSH:

```toml
basalto-shared = { git = "ssh://git@github.com/ShadeC0der/basalto-shared.git", tag = "v1.2.0" }
```

HTTPS:

```toml
basalto-shared = { git = "https://github.com/ShadeC0der/basalto-shared", tag = "v1.2.0" }
```
