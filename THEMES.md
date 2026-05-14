# Theme discovery

Ferrisume looks for themes in the following order:

- `./themes` in the current directory
- XDG data dir: `${XDG_DATA_HOME:-$HOME/.local/share}/ferrisume/themes`
- System dir (non-Windows): `/usr/share/ferrisume/themes`
- macOS: `~/Library/Application Support/com.ferrisume.ferrisume/themes`
- Windows: `%APPDATA%\ferrisume\ferrisume\data\themes`
- Next to the executable: `<exe-dir>/themes`

Quick setup (Linux/XDG):

```sh
mkdir -p "${XDG_DATA_HOME:-$HOME/.local/share}/ferrisume/themes/<your-theme>"
# Place `config.toml` and `templates/` inside that directory
```

Verify discovery:

```sh
RUST_LOG=info ferrisume themes
```

Notes

- If no theme named `default` is found on the filesystem, the embedded default is extracted to a temporary directory (you may see a `/tmp/.../default` path in the list).
- You can reference a theme by path:

```sh
ferrisume export -t /path/to/your/theme
```
