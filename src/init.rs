pub fn generate(shell: &str) -> String {
    match shell {
        "bash" => BASH_SCRIPT.to_string(),
        "zsh" => ZSH_SCRIPT.to_string(),
        other => unreachable!("unsupported shell: {other}"),
    }
}

// dir=$(hatoba select) && cd "${dir}" では失敗する事象を確認。
// ${CARGO_HOME} で解消したものの、cargo 以外でインストールした hatoba では、コマンドが見つからない可能性がある。
const BASH_SCRIPT: &str = r#"_hatoba_hook() {
  if [[ -t 0 && -t 1 ]]; then
    local dir
    dir=$(${CARGO_HOME:-$HOME/.cargo}/bin/hatoba select) && cd "${dir}"
  fi
}
[[ "$0" == "-bash" ]] && _hatoba_hook
"#;

const ZSH_SCRIPT: &str = r#"_hatoba_hook() {
  if [[ -o interactive && -t 0 && -t 1 ]]; then
    local dir
    dir=$(${CARGO_HOME:-$HOME/.cargo}/bin/hatoba select) && cd "${dir}"
  fi
}
[[ -o login ]] && _hatoba_hook
"#;
