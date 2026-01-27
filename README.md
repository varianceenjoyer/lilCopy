# lilCopy

> *Seize the memes of production, comrade!*

A lightweight Windows shell extension that adds a right-click option to copy a media file's **thumbnail** to your clipboard instead of the full-size image. Perfect for Discord-friendly pasting of older, larger memes.

![Context Menu Example](docs/context-menu.png)

## Features

- 🖼️ Extracts Windows Explorer thumbnails (128px default)
- 📋 Copies directly to clipboard as image
- ⚡ Fast - uses native Windows Shell API
- 🦀 Written in Rust for safety and performance
- 📦 Tiny binary (~500KB)

## Supported Formats

Images: `.jpg`, `.jpeg`, `.png`, `.gif`, `.webp`, `.bmp`
Videos: `.mp4`, `.webm`, `.mov`, `.avi`, `.mkv`

## Installation

### Prerequisites

- Windows 10/11
- [Rust toolchain](https://rustup.rs/) (for building)

### Build & Install

```powershell
# Clone the repo
git clone https://github.com/yourname/lilcopy
cd lilcopy

# Build release binary
cargo build --release

# Install (no admin required - uses per-user registry)
.\install.ps1
```

### Uninstall

```powershell
.\install.ps1 -Uninstall
```

## Usage

1. Right-click any image or video file in Explorer
2. Click **"Copy Thumbnail 📋"**
3. Paste anywhere (Ctrl+V)

### Command Line

```powershell
# Copy thumbnail at default size (128px)
lilcopy "C:\memes\ancient_meme.jpg"

# Copy thumbnail at custom size
lilcopy "C:\memes\ancient_meme.jpg" 256
```

## How It Works

```
┌─────────────────┐     ┌──────────────────┐     ┌───────────┐
│  Right-click    │────▶│  Windows Shell   │────▶│ Clipboard │
│  context menu   │     │  IShellItem      │     │  (RGBA)   │
│                 │     │  ImageFactory    │     │           │
└─────────────────┘     └──────────────────┘     └───────────┘
```

lilCopy uses the Windows Shell API (`IShellItemImageFactory`) to request the same thumbnails that Explorer displays. This means:

- Thumbnails are cached by Windows (fast!)
- Video thumbnails work automatically
- No image processing libraries needed for extraction

## Configuration

### Custom Thumbnail Size

Edit the registry value or pass size as second argument:

```powershell
lilcopy "image.png" 64   # Tiny
lilcopy "image.png" 128  # Default (Discord-friendly)
lilcopy "image.png" 256  # Larger
```

### Discord Optimization

Discord's inline image preview works best with images under 256px. The default 128px is chosen for optimal quality-to-size ratio.

## Building from Source

```powershell
# Debug build
cargo build

# Release build (optimized, smaller)
cargo build --release

# Run tests
cargo test
```

## Technical Details

- **Shell Integration**: Registers under `HKCU\Software\Classes\SystemFileAssociations\{image,video}\shell\lilCopy`
- **Thumbnail API**: Uses `IShellItemImageFactory::GetImage` with `SIIGBF_THUMBNAILONLY`
- **Clipboard Format**: RGBA bitmap via `arboard` crate
- **COM Threading**: Apartment-threaded (STA)

## Troubleshooting

### "Thumbnail not available"

Some files don't have cached thumbnails. Try:
1. Open the folder in Explorer to generate thumbnails
2. Use a larger size (some thumbnails only exist at certain sizes)

### Context menu doesn't appear

1. Make sure you ran `install.ps1` successfully
2. Restart Explorer: `taskkill /f /im explorer.exe && start explorer`

### Clipboard is empty after copy

Check if another application is locking the clipboard. Close clipboard managers temporarily.

## License

[MIT License](LICENSE) - Use freely, distribute widely, share the memes.

## Contributing

PRs welcome! Some ideas:

- [ ] Toast notification on success/failure
- [ ] Configurable default size via registry
- [ ] Batch mode for multiple files
- [ ] Custom thumbnail generation fallback

---

*Made with ☭ for the meme proletariat*
