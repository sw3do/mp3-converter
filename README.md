<div align="center">

# 🎵 MP3 Converter CLI

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![GitHub release](https://img.shields.io/github/release/sw3do/mp3-converter.svg?style=for-the-badge)](https://github.com/sw3do/mp3-converter/releases)
[![GitHub stars](https://img.shields.io/github/stars/sw3do/mp3-converter.svg?style=for-the-badge)](https://github.com/sw3do/mp3-converter/stargazers)

**A blazingly fast CLI tool for converting MP4 files to MP3 and downloading YouTube videos as MP3**

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Examples](#-examples) • [Contributing](#-contributing)

</div>

---

## ✨ Features

🎬 **Local MP4 Conversion** - Convert any MP4 file to high-quality MP3 format  
📺 **YouTube Download** - Download and convert YouTube videos directly to MP3  
🚀 **Lightning Fast** - Built with Rust for maximum performance  
🎯 **Smart Naming** - Automatic output file naming when not specified  
📁 **Auto Directory Creation** - Creates output directories automatically  
🛡️ **Error Handling** - Comprehensive error messages and validation  
⚙️ **High Quality Audio** - 192kbps bitrate, 44.1kHz sample rate  

## 🚀 Installation

### 📦 Download Pre-built Binaries (Recommended)

**Latest Release: v0.1.0**

Download the latest pre-built binary for your platform:

| Platform | Architecture | Download Link |
|----------|-------------|---------------|
| 🍎 **macOS** | Apple Silicon (M1/M2) | [mp3-converter](https://github.com/sw3do/mp3-converter/releases/latest/download/mp3-converter) |
| 🍎 **macOS** | Intel (x64) | [mp3-converter](https://github.com/sw3do/mp3-converter/releases/latest/download/mp3-converter) |
| 🐧 **Linux** | x64 | [mp3-converter](https://github.com/sw3do/mp3-converter/releases/latest/download/mp3-converter) |
| 🪟 **Windows** | x64 | [mp3-converter.exe](https://github.com/sw3do/mp3-converter/releases/latest/download/mp3-converter.exe) |

#### Quick Install (macOS/Linux)
```bash
# Download and install (replace URL with your platform)
curl -L https://github.com/sw3do/mp3-converter/releases/latest/download/mp3-converter -o mp3-converter
chmod +x mp3-converter
sudo mv mp3-converter /usr/local/bin/

# Verify installation
mp3-converter --help
```

### Prerequisites

Before using this tool, install the following dependencies:

#### FFmpeg (Required for MP4 conversion)
```bash
# macOS
brew install ffmpeg

# Ubuntu/Debian
sudo apt update && sudo apt install ffmpeg

# Windows (Chocolatey)
choco install ffmpeg
```

#### yt-dlp (Required for YouTube downloads)
```bash
# Using pip
pip install yt-dlp

# macOS
brew install yt-dlp
```

### 🔨 Build from Source

```bash
# Clone the repository
git clone https://github.com/sw3do/mp3-converter.git
cd mp3-converter

# Build the project
cargo build --release

# The executable will be available at target/release/mp3-converter
```

### 🎯 Release Information

#### Current Version: v0.1.0
**Release Date:** January 2024

**What's New:**
- ✨ Initial release with core functionality
- 🎬 Local MP4 to MP3 conversion support
- 📺 YouTube video downloading and conversion
- 🚀 High-performance Rust implementation
- 🛡️ Comprehensive error handling
- 📖 Full CLI documentation

**Binary Details:**
- **Size:** ~1.4MB (optimized release build)
- **Dependencies:** FFmpeg, yt-dlp (external)
- **Platforms:** macOS (ARM64/x64), Linux (x64), Windows (x64)
- **Rust Version:** 1.75+

#### Previous Releases
- This is the initial release

#### Upcoming Features (v0.2.0)
- 🎵 Support for additional audio formats (FLAC, AAC)
- 📁 Batch processing capabilities
- 🎛️ Custom audio quality settings
- 🔄 Progress bars for long operations
- 🌐 Playlist download support

## 📖 Usage

### Convert Local MP4 to MP3

```bash
# Convert with automatic output filename
./target/release/mp3-converter convert -i video.mp4

# Convert with custom output filename
./target/release/mp3-converter convert -i video.mp4 -o audio.mp3
```

### Download YouTube Video as MP3

```bash
# Download to current directory
./target/release/mp3-converter download -u "https://www.youtube.com/watch?v=VIDEO_ID"

# Download to specific directory
./target/release/mp3-converter download -u "https://www.youtube.com/watch?v=VIDEO_ID" -o ./downloads
```

### Get Help

```bash
# General help
./target/release/mp3-converter --help

# Command-specific help
./target/release/mp3-converter convert --help
./target/release/mp3-converter download --help
```

## 💡 Examples

<details>
<summary>Click to see more examples</summary>

### Batch Convert Multiple Files
```bash
# Convert all MP4 files in a directory
for file in *.mp4; do
    ./target/release/mp3-converter convert -i "$file"
done
```

### Download Playlist (using yt-dlp directly)
```bash
# For playlists, use yt-dlp directly with our tool's settings
yt-dlp -x --audio-format mp3 --audio-quality 192K "PLAYLIST_URL"
```

### Custom Output Directory
```bash
# Create and use custom output directory
mkdir -p ~/Music/converted
./target/release/mp3-converter convert -i video.mp4 -o ~/Music/converted/audio.mp3
```

</details>

## ⚙️ Audio Quality Settings

| Setting | Value | Description |
|---------|-------|-------------|
| **Bitrate** | 192kbps | High quality audio |
| **Sample Rate** | 44.1kHz | CD quality |
| **Format** | MP3 | Universal compatibility |

## 🔧 Troubleshooting

<details>
<summary>Common Issues and Solutions</summary>

### "FFmpeg not found" error
- Ensure FFmpeg is installed and available in your system PATH
- Try running `ffmpeg -version` to verify installation

### "yt-dlp not found" error
- Install yt-dlp using `pip install yt-dlp`
- Verify installation with `yt-dlp --version`

### Permission errors
- Check write permissions for the output directory
- Use `chmod` or run with appropriate permissions

### YouTube download fails
- Update yt-dlp: `pip install --upgrade yt-dlp`
- Some videos may be geo-restricted or private

</details>

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [FFmpeg](https://ffmpeg.org/) - For audio/video processing
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - For YouTube downloading capabilities
- [Clap](https://github.com/clap-rs/clap) - For CLI argument parsing

---

<div align="center">

**Made with ❤️ by [sw3do](https://github.com/sw3do)**

⭐ Star this repository if you found it helpful!

</div>