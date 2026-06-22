pub struct FlagHelp {
    pub name: &'static str,
    pub description: &'static str,
}

pub struct CommandHelp {
    pub name: &'static str,
    pub description: &'static str,
    pub flags: &'static [FlagHelp],
}

// Contrato de comportamiento para todos los plugins
pub trait BasaltoPlugin {
    // Necesita un nombre unico del plugin
    fn name(&self) -> &str;

    // Necesita mostrar los comandos que tiene
    fn plugin_commands(&self) -> &[&str];

    // Necesita preparar los recursos del plugin
    fn on_load(&self);

    // Necesita ejecutar los comandos del plugin (comando + argumentos)
    fn execute_command(&self, command: &str, args: &[&str]);

    // Retorna la ayuda de cada comando con sus flags (implementacion opcional)
    fn command_help(&self) -> &'static [CommandHelp] {
        &[]
    }
}
