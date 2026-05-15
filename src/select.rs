use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{self, Color},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

use crate::config::{Config, Dir};

// ヘッダー行数: "=== ... ===" + "↑↓ ..." + 空行 の 3 行
const HEADER_LINES: u16 = 3;

pub fn run(config: &Config) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let dirs = &config.dirs;
    let default_path = config.default.as_deref();

    let mut selected = default_path
        .and_then(|d| dirs.iter().position(|dir| dir.path == d))
        .unwrap_or(0);

    let mut stderr = io::stderr();

    terminal::enable_raw_mode()?;

    draw_menu(&mut stderr, dirs, selected, default_path)?;

    let chosen = loop {
        match event::read() {
            Err(e) => {
                // raw モードを必ず解除してからエラーを伝播する
                let _ = terminal::disable_raw_mode();
                return Err(e.into());
            }
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Up,
                kind: KeyEventKind::Press,
                ..
            })) => {
                selected = if selected == 0 {
                    dirs.len() - 1
                } else {
                    selected - 1
                };
                redraw_menu(&mut stderr, dirs, selected, default_path)?;
            }
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                ..
            })) => {
                selected = (selected + 1) % dirs.len();
                redraw_menu(&mut stderr, dirs, selected, default_path)?;
            }
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            })) => {
                break Some(dirs[selected].path.clone());
            }
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Esc,
                kind: KeyEventKind::Press,
                ..
            })) => {
                break None;
            }
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                ..
            })) => {
                break None;
            }
            _ => {}
        }
    };

    terminal::disable_raw_mode()?;
    clear_menu(&mut stderr, dirs.len())?;

    Ok(chosen)
}

fn draw_menu(
    stderr: &mut impl Write,
    dirs: &[Dir],
    selected: usize,
    default: Option<&str>,
) -> io::Result<()> {
    queue!(
        stderr,
        style::Print("=== hatoba: 作業ディレクトリを選択 ===\r\n"),
        style::Print("↑↓ で移動、Enter で確定、Esc でキャンセル\r\n"),
        style::Print("\r\n"),
    )?;
    for (i, dir) in dirs.iter().enumerate() {
        render_row(stderr, dir, i == selected, default)?;
    }
    stderr.flush()
}

fn redraw_menu(
    stderr: &mut impl Write,
    dirs: &[Dir],
    selected: usize,
    default: Option<&str>,
) -> io::Result<()> {
    let total = HEADER_LINES + dirs.len() as u16;
    execute!(
        stderr,
        cursor::MoveUp(total),
        terminal::Clear(ClearType::FromCursorDown),
    )?;
    draw_menu(stderr, dirs, selected, default)
}

fn clear_menu(stderr: &mut impl Write, n_dirs: usize) -> io::Result<()> {
    let total = HEADER_LINES + n_dirs as u16;
    execute!(
        stderr,
        cursor::MoveUp(total),
        terminal::Clear(ClearType::FromCursorDown),
    )
}

fn render_row(
    stderr: &mut impl Write,
    dir: &Dir,
    is_selected: bool,
    default: Option<&str>,
) -> io::Result<()> {
    let cursor_char = if is_selected { "▶" } else { " " };
    let default_mark = if default == Some(dir.path.as_str()) {
        "  (default)"
    } else {
        ""
    };
    let line = format!("{} {}{}\r\n", cursor_char, dir.display(), default_mark);

    if is_selected {
        queue!(
            stderr,
            style::SetForegroundColor(Color::Cyan),
            style::Print(line),
            style::ResetColor,
        )
    } else {
        queue!(stderr, style::Print(line))
    }
}
