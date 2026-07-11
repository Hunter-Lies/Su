<p align="center"><img src="logo.png" width="128" alt="Su!"></p>

<h1 align="center">Su! — 局域网文件快传</h1>
<p align="center"><em>咻一下，就到了！</em></p>

<p align="center">
  <img src="screenshot.png" width="600" alt="截图">
</p>

<p align="center">
  <a href="https://github.com/Hunter-Lies/Su/releases"><img src="https://img.shields.io/github/v/release/Hunter-Lies/Su?style=flat-square" alt="Release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/Hunter-Lies/Su?style=flat-square" alt="MIT"></a>
  <a href="README.md">English</a>
</p>

---

轻量级局域网文件传输工具。**无需云端、无需登录、无需安装 App**，手机扫码即可与电脑互传文件。

## 功能特性

- **二维码分享** — 右键任意文件 → 生成二维码 → 手机扫码即下载
- **手机传电脑** — 手机浏览器打开网页，选择文件发送到电脑
- **多文件批量** — 全选 / 单选 / 批量下载，文件清单清晰
- **5 种语言** — 简体中文 · 繁體中文 · English · 日本語 · 한국어
- **深色模式** — 经典 / 毛玻璃主题，自动跟随系统
- **智能批次** — 自动识别设备 (iPhone / Android / Windows / Mac / Linux)，按批次分组
- **提示音 + 弹窗** — 收到文件时播放音效、弹出卡片通知
- **系统托盘** — 关闭到托盘、开机自启
- **断点续传** — Range 分块下载，大文件无忧

## 快速开始

1. 下载 [最新 Release](https://github.com/Hunter-Lies/Su/releases)
2. 双击 `Su-v1.2.0-windows-x64.exe`（或 x86 版本）
3. 拖文件到窗口 → 手机扫码下载
4. 手机端扫码 → 选择文件发到电脑

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面应用 | [Tauri v2](https://tauri.app/) + Rust |
| 前端 UI | Vanilla JS (ES Modules) + CSS 自定义属性 |
| 手机端页面 | 纯 HTML/JS/CSS，由内置 HTTP 服务器提供 |
| HTTP 服务 | [tiny_http](https://github.com/tiny-http/tiny-http) |
| 二维码 | [qrcode](https://crates.io/crates/qrcode) |
| 音频 | [rodio](https://github.com/RustAudio/rodio) + [symphonia](https://github.com/pdeljanov/Symphonia) |

## 构建

**环境要求**

| 工具 | 版本 |
|------|------|
| [Node.js](https://nodejs.org/) | >= 18 |
| [Rust](https://rustup.rs/) | >= 1.70 |
| WebView2 | Windows 10/11 已内置 |

```bash
npm install
npm run tauri dev                    # 开发模式
npm run tauri build                  # 构建 64 位
npm run tauri build -- --target i686-pc-windows-msvc  # 构建 32 位
```

构建产物在 `src-tauri/target/` 下。

## 项目结构

```
Su/
├── src/                      # 桌面前端
│   ├── index.html
│   ├── js/                   # main.js, i18n.js, theme.js, ...
│   ├── css/styles.css
│   └── assets/fonts/         # Font Awesome（本地）
├── src-tauri/                # Rust 后端
│   ├── src/                  # Rust 源码
│   ├── web/                  # 手机端页面
│   ├── sounds/               # 提示音文件
│   └── icons/                # 应用图标
├── package.json
└── README.md
```

## 贡献者

[![Contributors](https://contrib.rocks/image?repo=Hunter-Lies/Su)](https://github.com/Hunter-Lies/Su/graphs/contributors)

## 许可证

MIT — 详见 [LICENSE](LICENSE)

## 作者

**HunterLies** · [B站](https://space.bilibili.com/488494586) · [官网](https://htovo.com)
