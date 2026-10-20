//! kconfig, spelled once. Writing a KDE setting is always the same five-argument shape, and every
//! caller used to spell it out again, tool name and all. Here it is one place, so the argument
//! order, the "is the tool there?" question and the elevated variant cannot drift apart.

use crate::privileges;
use crate::process;

/// Whether kconfig's tools are on this machine. Every caller that used to check for itself asks
/// this instead.
pub fn available() -> bool {
    process::available("kwriteconfig6")
}

/// The lookup path kconfig walks, outermost group first.
fn arguments<'a>(file: &'a str, groups: &'a [&'a str], key: &'a str) -> Vec<&'a str> {
    let mut args = vec!["--file", file];
    for group in groups {
        args.push("--group");
        args.push(group);
    }
    args.push("--key");
    args.push(key);
    args
}

/// Set one key. Returns whether the write went through.
pub fn write(file: &str, groups: &[&str], key: &str, value: &str) -> bool {
    let mut args = arguments(file, groups, key);
    args.push(value);
    process::succeeds("kwriteconfig6", &args, None)
}

/// Set one key in one group, which is most of the calls.
pub fn write_key(file: &str, group: &str, key: &str, value: &str) -> bool {
    write(file, &[group], key, value)
}

/// Drop one key, so the setting falls back to its default.
pub fn delete(file: &str, groups: &[&str], key: &str) -> bool {
    let mut args = arguments(file, groups, key);
    args.push("--delete");
    process::succeeds("kwriteconfig6", &args, None)
}

/// Read one key. `None` when it is unset, empty, or the tool is missing.
pub fn read(file: &str, groups: &[&str], key: &str) -> Option<String> {
    let args = arguments(file, groups, key);
    process::capture("kreadconfig6", &args).filter(|value| !value.is_empty())
}

/// Set one key in a file only root may write, such as `/etc/sddm.conf`.
pub fn write_elevated(file: &str, groups: &[&str], key: &str, value: &str) -> bool {
    let mut args = arguments(file, groups, key);
    args.push(value);
    privileges::elevated("kwriteconfig6", &args, None).is_ok_and(|code| code == 0)
}

/// Drop one key from a file only root may write.
pub fn delete_elevated(file: &str, groups: &[&str], key: &str) -> bool {
    let mut args = arguments(file, groups, key);
    args.push("--delete");
    privileges::elevated("kwriteconfig6", &args, None).is_ok_and(|code| code == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_write_names_the_file_the_groups_and_the_key_in_that_order() {
        assert_eq!(
            arguments("kwinrc", &["Desktops"], "Number"),
            ["--file", "kwinrc", "--group", "Desktops", "--key", "Number"]
        );
    }

    #[test]
    fn nested_groups_are_walked_outermost_first() {
        assert_eq!(
            arguments("kdeglobals", &["General", "Preview"], "Enable"),
            [
                "--file",
                "kdeglobals",
                "--group",
                "General",
                "--group",
                "Preview",
                "--key",
                "Enable"
            ]
        );
    }
}
