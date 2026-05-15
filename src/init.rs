pub fn generate(shell: &str) -> Result<String, String> {
    match shell {
        "bash" => Ok(BASH_SCRIPT.to_string()),
        "zsh" => Ok(ZSH_SCRIPT.to_string()),
        other => Err(format!(
            "unsupported shell: '{}'. Use 'bash' or 'zsh'",
            other
        )),
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
