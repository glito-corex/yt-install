# Dependencies & Download Engines

YT_install uses two download engines: **IXD** and **yt-dlp**. Their installation methods are different.

## 1. IXD — Included with YT_install

IXD (Internet Xtreme Downloader) is directly included in the YT_install project.

The original IXD implementation has been modified and adapted for YT_install, with the goal of reducing unnecessary components and keeping the application lightweight.

The modified version is bundled locally in:

```text
Lib/IXD/
```

You do not need to download or install IXD separately.

**Important:** This is a modified version of IXD, not necessarily the complete original distribution.

## 2. yt-dlp — Download required

Unlike IXD, yt-dlp is not included directly in the source distribution of YT_install.

You must download it separately from its official GitHub repository:

**Official repository:**  
https://github.com/yt-dlp/yt-dlp

**Download page:**  
https://github.com/yt-dlp/yt-dlp/releases/latest

Download the appropriate release and place the required yt-dlp files in:

```text
Lib/YT-DLP/
```

The expected directory structure is:

```text
Lib/
└── YT-DLP/
    ├── yt_dlp/
    ├── bundle/
    └── yt-dlp.cmd
```

Make sure the downloaded version is compatible with YT_install.

## 3. Summary

| Component | Included? | Installation |
|---|---|---|
| IXD | Yes | Already integrated |
| yt-dlp | No | Must be downloaded separately |
| Python | Required for source execution | Install Python if not already available |

## 4. Official projects

- **IXD:** Internet Xtreme Downloader — modified and integrated into YT_install.
- **yt-dlp:** https://github.com/yt-dlp/yt-dlp

Please respect the respective licenses of each project when redistributing YT_install or its components.

