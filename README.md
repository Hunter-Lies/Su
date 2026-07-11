# Su! - LAN File Transfer

> ????????Whoosh, it's there!

A lightweight LAN file transfer tool. Share files between phone and PC by scanning a QR code. No cloud, no upload, pure local network.

## Features

- **QR Code Sharing** ? Right-click any file and share via QR. Phone scans and downloads instantly.
- **Phone to PC** ? Send files from phone browser to your computer over LAN.
- **Multi-file Support** ? Batch download with select-all and progress tracking.
- **Multi-language** ? Supports Chinese (Simplified/Traditional), English, Japanese, Korean.
- **Dark Mode** ? Classic and frosted glass themes with auto dark mode.
- **Batch Receive** ? Smart batch grouping with device identification (iPhone/Android/Windows/Mac/Linux).
- **Notification Sounds** ? Configurable sound alerts on file receive.
- **System Tray** ? Minimize to tray, close to tray options.
- **Resumable Upload** ? Chunked upload with progress tracking.
- **Privacy & Security** ? Optional access code for received files.

## Tech Stack

- **Desktop**: Tauri v2 + Rust
- **Frontend**: Vanilla JS (ES Modules) + CSS Custom Properties
- **Mobile Pages**: Plain HTML/JS/CSS served by embedded HTTP server
- **HTTP Server**: tiny_http (Rust)

## Project Structure

```
Su/
??? src/                    # Desktop frontend
?   ??? index.html          # Main window
?   ??? js/                 # JavaScript modules
?   ?   ??? main.js         # Entry point
?   ?   ??? i18n.js         # Internationalization
?   ?   ??? state.js        # DOM refs & app state
?   ?   ??? utils.js        # Toast, clipboard, QR helpers
?   ?   ??? settings.js     # Settings page
?   ?   ??? share.js        # File sharing UI
?   ?   ??? received.js     # Received records UI
?   ?   ??? theme.js        # Theme & appearance
?   ??? css/
?   ?   ??? styles.css      # All styles
?   ??? assets/
?       ??? fonts/          # Font Awesome (local)
??? src-tauri/
?   ??? src/                # Rust backend
?   ?   ??? main.rs         # Entry point
?   ?   ??? lib.rs          # Tauri setup, window, tray
?   ?   ??? commands.rs     # Tauri IPC commands
?   ?   ??? http.rs         # Embedded HTTP server
?   ?   ??? state.rs        # Shared app state
?   ?   ??? utils.rs        # HTTP helpers
?   ?   ??? qr.rs           # QR code generation
?   ?   ??? sound.rs        # Sound notification
?   ?   ??? com_shellext.rs # Windows shell extension
?   ??? web/                # Mobile web pages (served by HTTP)
?   ?   ??? send.html       # Phone ? PC send page
?   ?   ??? download.html   # Single file download
?   ?   ??? bundle_multi.html # Multi-file download
?   ?   ??? i18n.js         # Mobile translations
?   ?   ??? fonts/          # Font Awesome
?   ??? sounds/             # Notification sound files
?   ??? icons/              # App icons
?   ??? Cargo.toml
?   ??? tauri.conf.json
??? LICENSE
??? README.md
```

## Development

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (for Tauri CLI)
- Windows: Visual Studio Build Tools (for `windows-rs`)

### Build
```bash
cd src-tauri
cargo build          # Debug
cargo build --release # Release
```

### Run
```bash
cargo run
```

## Author

**HunterLies**

- Website: [Htovo.com](https://htovo.com)
- Bilibili: [@HunterLies](https://space.bilibili.com/488494586)

## License

MIT
