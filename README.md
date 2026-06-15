# basalto-shared

Plugin contract for the Basalto ecosystem. Defines the `BasaltoPlugin` trait.

## Uso

Un plugin es un crate de Rust que compila a una librería dinámica `.so` e implementa `BasaltoPlugin`.

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

#[no_mangle]
pub extern "C" fn _basalto_create_plugin() -> *mut dyn BasaltoPlugin {
    Box::into_raw(Box::new(MyPlugin))
}
```

`#[no_mangle]` y `extern "C"` son necesarios para que basalto-core pueda encontrar el constructor por nombre dentro del `.so` en tiempo de ejecución.

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

#[no_mangle]
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
