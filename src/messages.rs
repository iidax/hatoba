use crate::config::Language;

pub struct Msg {
    pub added: &'static str,
    pub removed: &'static str,
    pub default_set: &'static str,
    pub swapped: &'static str,
    pub select_prompt: &'static str,
    pub default_marker: &'static str,
    pub remove_prompt: &'static str,
    pub default_prompt: &'static str,
    pub lang_prompt: &'static str,
    pub swap_prompt_first: &'static str,
    pub swap_prompt_second: &'static str,
}

pub static EN: Msg = Msg {
    added: "Added",
    removed: "Removed",
    default_set: "Default set to",
    swapped: "Swapped",
    select_prompt: "hatoba: Select working directory  (↑/↓ j/k to move, Enter/Space to confirm, Esc/q to cancel",
    default_marker: "  (default)",
    remove_prompt: "hatoba: Select directory to remove  (↑/↓ j/k to move, Enter/Space to confirm, Esc/q to cancel",
    default_prompt: "hatoba: Select directory to set as default  (↑/↓ j/k to move, Enter/Space to confirm, Esc/q to cancel",
    lang_prompt: "hatoba: Select language  (↑/↓ j/k to move, Enter/Space to confirm, Esc/q to cancel",
    swap_prompt_first: "hatoba: Swap — select first directory  (↑/↓ j/k to move, Enter/Space to confirm, Esc/q to cancel",
    swap_prompt_second: "hatoba: Swap — select second directory  (↑/↓ j/k to move, Enter/Space to confirm, Esc/q to cancel",
};

pub static JA: Msg = Msg {
    added: "追加しました",
    removed: "削除しました",
    default_set: "デフォルトを変更しました",
    swapped: "並び替えました",
    select_prompt: "hatoba: 作業ディレクトリを選択  (↑/↓ j/k で移動、Enter/Space で決定、Esc/q でキャンセル",
    default_marker: "  (デフォルト)",
    remove_prompt: "hatoba: 削除するディレクトリを選択  (↑/↓ j/k で移動、Enter/Space で決定、Esc/q でキャンセル",
    default_prompt: "hatoba: デフォルトに設定するディレクトリを選択  (↑/↓ j/k で移動、Enter/Space で決定、Esc/q でキャンセル",
    lang_prompt: "hatoba: 言語を選択  (↑/↓ j/k で移動、Enter/Space で決定、Esc/q でキャンセル",
    swap_prompt_first: "hatoba: 並び替え — 1つ目のディレクトリを選択  (↑/↓ j/k で移動、Enter/Space で決定、Esc/q でキャンセル",
    swap_prompt_second: "hatoba: 並び替え — 2つ目のディレクトリを選択  (↑/↓ j/k で移動、Enter/Space で決定、Esc/q でキャンセル",
};

pub fn get(lang: &Language) -> &'static Msg {
    match lang {
        Language::En => &EN,
        Language::Ja => &JA,
    }
}
