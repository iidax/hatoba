use crate::config::Language;

pub struct Msg {
    pub added: &'static str,
    pub removed: &'static str,
    pub default_set: &'static str,
    pub swapped: &'static str,
    pub select_prompt: &'static str,
    pub default_marker: &'static str,
}

pub static EN: Msg = Msg {
    added: "Added",
    removed: "Removed",
    default_set: "Default set to",
    swapped: "Swapped",
    select_prompt: "hatoba: Select working directory  (↑/↓ j/k to move, Enter/Space to confirm, Esc/q to cancel",
    default_marker: "  (default)",
};

pub static JA: Msg = Msg {
    added: "追加しました",
    removed: "削除しました",
    default_set: "デフォルトを変更しました",
    swapped: "並び替えました",
    select_prompt: "hatoba: 作業ディレクトリを選択  (↑/↓ j/k で移動、Enter/Space で決定、Esc/q でキャンセル",
    default_marker: "  (デフォルト)",
};

pub fn get(lang: &Language) -> &'static Msg {
    match lang {
        Language::En => &EN,
        Language::Ja => &JA,
    }
}
