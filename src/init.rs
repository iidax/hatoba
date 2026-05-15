pub fn generate(shell: &str) -> String {
    match shell {
        "bash" => BASH_SCRIPT.to_string(),
        "zsh" => ZSH_SCRIPT.to_string(),
        other => unreachable!("unsupported shell: {other}"),
    }
}

const BASH_SCRIPT: &str = r#"_hatoba_hook() {
  if [[ -n "${SSH_CONNECTION}" && -t 0 && -t 1 ]]; then
    local dir
    dir=$(hatoba select) && cd "${dir}"
  fi
}
[[ "$0" == "-bash" ]] && _hatoba_hook
"#;

const ZSH_SCRIPT: &str = r#"_hatoba_hook() {
  if [[ -n "${SSH_CONNECTION}" && -o interactive && -t 0 && -t 1 ]]; then
    local dir
    dir=$(hatoba select) && cd "${dir}"
  fi
}
[[ -o login ]] && _hatoba_hook
"#;
