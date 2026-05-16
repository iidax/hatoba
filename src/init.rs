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
