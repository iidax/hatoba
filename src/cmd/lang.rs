use dialoguer::Select;

use crate::config::{Config, Language};
use crate::messages::Msg;
use std::path::PathBuf;

pub fn run(
    config_path: Option<PathBuf>,
    lang: Option<Language>,
    msg: &Msg,
) -> Result<(), Box<dyn std::error::Error>> {
    match lang {
        Some(new_lang) => set_language(config_path, new_lang),
        None => {
            let current = Config::load(config_path.clone()).language;
            let langs = [Language::En, Language::Ja];
            let items = ["en", "ja"];
            let default_idx = langs.iter().position(|l| l == &current).unwrap_or(0);
            let selection = Select::new()
                .with_prompt(msg.lang_prompt)
                .items(items)
                .default(default_idx)
                .interact_opt()?;
            if let Some(i) = selection {
                set_language(config_path, langs[i].clone())?;
            }
            Ok(())
        }
    }
}

fn set_language(
    config_path: Option<PathBuf>,
    new_lang: Language,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = config_path.map(Ok).unwrap_or_else(Config::path_default)?;
    if !file_path.exists() {
        if let Some(dir) = file_path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(&file_path, "")?;
    }
    let content = std::fs::read_to_string(&file_path)?;
    let mut doc = content.parse::<toml_edit::DocumentMut>()?;
    let lang_str = new_lang.to_string();
    doc["settings"]["language"] = toml_edit::value(lang_str.as_str());
    std::fs::write(&file_path, doc.to_string())?;
    println!("{new_lang}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as IoWrite;

    fn make_config_file(content: &str) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(file, "{content}").unwrap();
        file
    }

    #[test]
    fn set_language_writes_to_config() {
        let file = make_config_file("");
        run(
            Some(file.path().to_path_buf()),
            Some(Language::Ja),
            &crate::messages::EN,
        )
        .unwrap();
        let config = Config::load(Some(file.path().to_path_buf()));
        assert_eq!(config.language, Language::Ja);
    }

    #[test]
    fn set_language_back_to_en() {
        let file = make_config_file("[settings]\nlanguage = \"ja\"\n");
        run(
            Some(file.path().to_path_buf()),
            Some(Language::En),
            &crate::messages::EN,
        )
        .unwrap();
        let config = Config::load(Some(file.path().to_path_buf()));
        assert_eq!(config.language, Language::En);
    }
}
