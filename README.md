# YT_install

**YT_install** is a lightweight audio and video downloader written in Python. It allows users to download media from YouTube and other supported websites using two different download engines: **yt-dlp** and **IXD (Internet Xtreme Downloader)**.

The project focuses on providing an alternative between two download engines, allowing users to choose which one they want to use.

## Features

- **Audio downloading:** Download audio from a URL using yt-dlp.
- **Video downloading:** Download videos using either yt-dlp or IXD.
- **Two download engines:** Switch between yt-dlp and IXD directly from the application menu.
- **Local downloads:** Downloaded files are saved automatically in the user's `Downloads/video` directory.
- **Simple interface:** A lightweight command-line menu for selecting the download type and engine.
- **No API key required:** The application uses its bundled download engines rather than requiring a paid API.

## Download engines

### yt-dlp

[yt-dlp](https://github.com/yt-dlp/yt-dlp) is an open-source media downloader.

YT_install uses it for:

- Audio downloads using the `bestaudio` format.
- Video downloads using yt-dlp's default format selection.
- Downloading media from supported websites.
- Handling media extraction and download processing.

### IXD — Internet Xtreme Downloader

IXD is an alternative download engine bundled with the project.

Its internal engine includes:

- Multithreaded downloading.
- HTTP range-based downloads.
- Segmented downloads.
- HLS and DASH extraction.
- Download pause and resume capabilities.
- Download management and transfer scheduling.
- HTTP connection management.

YT_install uses IXD through its command-line interface to process media URLs.

**Note:** IXD is currently available for video downloads in YT_install. The audio option is disabled when IXD is selected.

## Technologies and libraries

| Technology | Purpose |
|---|---|
| Python | Main programming language |
| yt-dlp | Audio and video extraction |
| IXD | Alternative download engine |
| subprocess | Launching download engines |
| pathlib | Managing file paths |
| os | Filesystem and system operations |
| sys | Python runtime management |
| time | Delays and menu timing |

The main application relies primarily on Python's standard library, alongside the two integrated download engines.

## Project structure

```text

├──build.rs
├──cargo.toml
├──icone.ico
├──src
|   ├──DACL.rs
|   ├──main.rs
|   ├──menuL.rs
|   ├──installer_music.rs
|   └──installer_video.rs
│
├── Lib/
│   ├── IXD/
│   │   ├── ixd/
│   │   └── launcher.pyw
│   │
│   └── YT-DLP/
│       ├── yt_dlp/
│       ├── bundle/
│       └── yt-dlp.cmd
│
└── LICENCE/
    ├── LICENSE-YT-install
    ├── LICENSE-IXD.md
    └── LICENSE-YT-DLP.md
```

## How it works

1. Launch YT_install.
2. Select audio or video downloading.
3. Enter the URL of the desired media.
4. The selected engine processes the URL.
5. The downloaded file is saved in `Downloads/video`.

The download engine can be changed directly from the main menu using the `lib` command.

## Licensing

YT_install, yt-dlp and IXD have separate license files.

Please refer to the `LICENCE` directory for the applicable licensing conditions.

## Author

Developed by **Corex Institut**.

Website: https://corexinstitut.netlify.app

