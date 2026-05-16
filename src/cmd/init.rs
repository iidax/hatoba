pub fn generate(shell: &str, bin: &str) -> String {
    match shell {
        "bash" => bash_script(bin),
        "zsh" => zsh_script(bin),
        other => unreachable!("unsupported shell: {other}"),
    }
}

fn bash_script(bin: &str) -> String {
    format!(
        r#"_hatoba_hook() {{
  if [[ -t 0 && -t 1 && "$PWD" == "$HOME" ]]; then
    local dir
    dir=$(command {bin} select) && cd "${{dir}}"
  fi
}}
[[ "$0" == "-bash" ]] && _hatoba_hook
"#
    )
}

fn zsh_script(bin: &str) -> String {
    format!(
        r#"_hatoba_hook() {{
  if [[ -o interactive && -t 0 && -t 1 && "$PWD" == "$HOME" ]]; then
    local dir
    dir=$(command {bin} select) && cd "${{dir}}"
  fi
}}
[[ -o login ]] && _hatoba_hook
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_bash_contains_bin_path() {
        let script = generate("bash", "/usr/local/bin/hatoba");
        assert!(script.contains("/usr/local/bin/hatoba select"));
    }

    #[test]
    fn generate_bash_checks_login_shell() {
        let script = generate("bash", "hatoba");
        assert!(script.contains(r#"[[ "$0" == "-bash" ]]"#));
    }

    #[test]
    fn generate_zsh_contains_bin_path() {
        let script = generate("zsh", "/usr/local/bin/hatoba");
        assert!(script.contains("/usr/local/bin/hatoba select"));
    }

    #[test]
    fn generate_zsh_checks_login_shell() {
        let script = generate("zsh", "hatoba");
        assert!(script.contains("-o login"));
    }

    #[test]
    fn generate_bash_and_zsh_differ() {
        let bash = generate("bash", "hatoba");
        let zsh = generate("zsh", "hatoba");
        assert_ne!(bash, zsh);
    }
}
