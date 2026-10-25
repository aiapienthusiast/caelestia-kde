//! Application Launcher Executor
//! Parity with `Launch.launchEntry` in `shell/modules/launcher/services/Apps.qml`.

use super::entry::DesktopEntry;
use std::process::Command;
use tracing::info;

pub struct AppExecutor;

impl AppExecutor {
    /// Clean Freedesktop field codes and build command arguments
    pub fn build_command(exec: &str, run_in_terminal: bool) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut quote_char = ' ';

        for c in exec.chars() {
            match c {
                '"' | '\'' if !in_quotes => {
                    in_quotes = true;
                    quote_char = c;
                }
                '"' | '\'' if in_quotes && c == quote_char => {
                    in_quotes = false;
                }
                ' ' | '\t' if !in_quotes => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }
        if !current.is_empty() {
            tokens.push(current);
        }

        // Filter out Freedesktop field codes (%f, %u, %F, %U, etc.)
        let cleaned: Vec<String> = tokens
            .into_iter()
            .filter(|token| !token.starts_with('%'))
            .collect();

        if run_in_terminal && !cleaned.is_empty() {
            // Check terminal env or default to kitty / xterm
            let terminal = std::env::var("TERMINAL").unwrap_or_else(|_| "kitty".to_string());
            let mut terminal_cmd = vec![terminal, "-e".to_string()];
            terminal_cmd.extend(cleaned);
            terminal_cmd
        } else {
            cleaned
        }
    }

    /// Launch application process detached
    pub fn launch(entry: &DesktopEntry) -> Result<(), String> {
        let cmd_parts = Self::build_command(&entry.exec, entry.run_in_terminal);
        if cmd_parts.is_empty() {
            return Err("Empty exec command".to_string());
        }

        info!("Launching application {}: {:?}", entry.name, cmd_parts);

        let program = &cmd_parts[0];
        let args = &cmd_parts[1..];

        Command::new(program)
            .args(args)
            .spawn()
            .map_err(|e| format!("Failed to spawn {program}: {e}"))?;

        Ok(())
    }
}
