# hatoba 🛳️

An interactive working directory selector plugin for Bash/Zsh — triggered on terminal startup or SSH login.

## Overview

`hatoba` displays a directory selection menu on SSH login, letting you navigate with arrow keys and `cd` into your chosen working directory.

- ✅ Select directories with arrow keys (↑↓)
- ✅ Manage options and defaults via `config.toml`
- ✅ Activates only on SSH + interactive + login shell

## Installation

```bash
cargo install hatoba
```

## Setup

### 1. Integrate with your shell

**For zsh users**

Run the following command:

```
echo 'eval "$(hatoba init zsh)"' >> ~/.zshrc
```

Or add the following directly to `~/.zshrc`:

```zsh
eval "$(hatoba init zsh)"
```

After editing, restart your shell or run `source ~/.zshrc`.

**For bash users**

Run the following command:

```bash
echo 'eval "$(hatoba init bash)"' >> ~/.bashrc
```

Or add the following directly to `~/.bashrc`:

```bash
eval "$(hatoba init bash)"
```

After editing, restart your shell or run `source ~/.bashrc`.

### 2. Verify

Confirm that hatoba is available:

```bash
# Check version
hatoba --version

# Check help
hatoba --help
```

### 3. Register directories

```bash
# Add a candidate (the config file is created automatically on first run)
hatoba add ~/Workspace/myproject --label myproject --default

# Add more candidates
hatoba add ~/Workspace/other --label other

# List registered entries
hatoba list
```

You can also change the default selection:

```bash
# Change the default selection
hatoba default ~/Workspace/foo
```

### Notes

You can also edit `~/.config/hatoba/config.toml` directly:

```toml
# ~/.config/hatoba/config.toml
[[dirs]]
path = "~/Workspace/myproject"
label = "myproject"
default = true

[[dirs]]
path = "~/Workspace/other"
label = "other"
```

---

## Command Reference

| Command | Description |
|---|---|
| `hatoba list` | List registered directories |
| `hatoba add <path> [--label <name>] [--default]` | Add a directory |
| `hatoba remove <path>` | Remove a directory |
| `hatoba default <path>` | Change the default selection |
