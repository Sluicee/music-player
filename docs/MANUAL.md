# User Manual

This guide explains how "Memory Card" recognizes your music collection and the best way to organize your files for the best experience.

## File Recognition

### Supported Formats
The program supports the following file extensions:
- **Audio:** `.mp3`, `.flac`, `.ogg`, `.m4a`, `.aac`, `.wav`, `.opus`.
- **Covers:** `.jpg`, `.jpeg`, `.png`.

### Automatic Grouping
The player groups tracks into albums using the metadata (tags) inside your files:
1. **Albums:** Grouping is based on the **"Album"** and **"Album Artist"** tags. If "Album Artist" is missing, the primary "Artist" is used.
2. **Multi-disc Albums:** If an album title includes suffixes like "(Disc 1)" or "[CD 2]", the program automatically hides these for grouping. The entire album appears as a single entry, with tracks sorted by disc number and then track number.
3. **Guest Features (Feat.):** When determining the album owner, the program ignores common suffixes like `feat.`, `ft.`, or `featuring` in the artist name. This ensures an artist's discography stays together even if some tracks list guest performers.

### Album Covers
The program looks for cover art in the following order:
1. **Embedded Tags:** Checks for images embedded directly within the audio file.
2. **Folder Files:** If no embedded cover is found, it searches the file's directory for images named: `cover`, `folder`, `front`, `album`, `albumart`, or `artwork`.
3. **First Available Image:** If none of the above are found, the first image file in the directory will be used.

> [!TIP]
> For the best performance and visual quality, recommended cover size is **600x600 pixels**. The program automatically optimizes large images during scanning.

---

## Library Organization Tips

For the fastest and most reliable scanning, we recommend the following folder structure:
```text
Music/
└── Artist/
    └── [Year] Album Name/
        ├── cover.jpg (recommended)
        ├── 01 - Track Name.flac
        └── 02 - Track Name.flac
```
While the program can handle files placed in a single directory (using tags), structured storage makes it easier to manage your collection.

---

## Controls & Hotkeys

| Action | Key |
| :--- | :--- |
| **Play / Pause** | `Space` |
| **Previous Track** | `←` (Left Arrow) |
| **Next Track** | `→` (Right Arrow) |
| **Volume Up** | `↑` (Up Arrow) |
| **Volume Down** | `↓` (Down Arrow) |
| **Mute / Unmute** | `M` |
| **Shuffle (Square ⏹)** | `S` (Shuffle all tracks) |
| **Repeat (Triangle 🔼)** | `R` (Toggle: One / All / None) |
| **Search (Circle ⏺)** | `F` or `/` (Open search) |
| **Back / Close** | `Escape` (Close overlays and windows) |
| **Switch Tabs** | `1` — Library, `2` — Playlists |

