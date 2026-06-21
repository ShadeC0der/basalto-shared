# basalto-shared

Plugin contract for the Basalto ecosystem. Defines the `BasaltoPlugin` trait.

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
basalto-shared = { git = "ssh://git@github.com/ShadeC0der/basalto-shared.git", tag = "v1.0.0" }
```

**`src/lib.rs`**

```rust
use basalto_shared::BasaltoPlugin;

struct MyPlugin;

impl BasaltoPlugin for MyPlugin {
    fn name(&self) -> &str {
        "my-plugin"
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

## Estructura recomendada

`execute_command` es el punto de entrada — la lógica real de cada
comando vive en un archivo separado:

**`src/lib.rs`** — enrutador puro, sin lógica

```rust
mod commands;

use basalto_shared::BasaltoPlugin;

struct MyPlugin;

impl BasaltoPlugin for MyPlugin {
    fn name(&self) -> &str { "my-plugin" }

    fn plugin_commands(&self) -> &[&str] { &["show", "add"] }

    fn on_load(&self) {}

    fn execute_command(&self, command: &str, args: &[&str]) {
        match command {
            "show" => commands::show(args),
            "add"  => commands::add(args),
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

**`src/commands.rs`** — lógica real de cada comando

```rust
pub fn show(args: &[&str]) {
    // show logic
}

pub fn add(args: &[&str]) {
    // add logic
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
basalto-shared = { git = "ssh://git@github.com/ShadeC0der/basalto-shared.git", tag = "v1.0.0" }
```

HTTPS:

```toml
basalto-shared = { git = "https://github.com/ShadeC0der/basalto-shared", tag = "v1.0.0" }
```
