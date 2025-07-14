# 🚀 Release Guide for MP3 Converter CLI

This document provides comprehensive information about releases, including how to create them and what each release contains.

## 📋 Current Release: v0.1.0

### Release Information
- **Version:** v0.1.0
- **Release Date:** January 2024
- **Build Type:** Optimized Release
- **Binary Size:** ~1.4MB
- **Rust Version:** 1.75+

### 🎯 What's Included

#### Core Features
- ✅ Local MP4 to MP3 conversion
- ✅ YouTube video downloading and conversion to MP3
- ✅ Command-line interface with clap
- ✅ Async/await support with Tokio
- ✅ High-quality audio output (192kbps, 44.1kHz)
- ✅ Automatic file naming
- ✅ Directory creation
- ✅ Comprehensive error handling

#### Technical Specifications
- **Language:** Rust 2024 Edition
- **Dependencies:** clap, tokio
- **External Tools:** FFmpeg, yt-dlp
- **Platforms:** macOS (ARM64/Intel), Linux (x64), Windows (x64)

## 🔨 Building Release Binaries

### Prerequisites
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cross-compilation targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
```

### Build Commands

#### macOS (Current Platform)
```bash
# Apple Silicon (ARM64)
cargo build --release --target aarch64-apple-darwin

# Intel (x64)
cargo build --release --target x86_64-apple-darwin
```

#### Linux
```bash
# x64
cargo build --release --target x86_64-unknown-linux-gnu
```

#### Windows
```bash
# x64 (requires mingw-w64)
cargo build --release --target x86_64-pc-windows-gnu
```

### Binary Locations
After building, binaries will be located at:
- `target/release/mp3-converter` (macOS)
- `target/x86_64-pc-windows-gnu/release/mp3-converter.exe` (Windows)

## 📦 Creating GitHub Releases

### Step 1: Prepare Release Assets
```bash
# Create release directory
mkdir -p release-assets

# Copy and rename binaries
cp target/release/mp3-converter release-assets/mp3-converter
cp target/x86_64-pc-windows-gnu/release/mp3-converter.exe release-assets/mp3-converter.exe

# Create checksums
cd release-assets
shasum -a 256 * > checksums.txt
```

### Step 2: Create Git Tag
```bash
# Create and push tag
git tag -a v0.1.0 -m "Release v0.1.0: Initial release with MP4 conversion and YouTube download support"
git push origin v0.1.0
```

### Step 3: GitHub Release
1. Go to your repository on GitHub
2. Click "Releases" → "Create a new release"
3. Choose the tag `v0.1.0`
4. Fill in the release information:

#### Release Title
```
🎵 MP3 Converter CLI v0.1.0 - Initial Release
```

#### Release Description
```markdown
## 🎉 Initial Release

Welcome to the first release of MP3 Converter CLI! This powerful tool allows you to convert MP4 files to MP3 and download YouTube videos directly as MP3 files.

### ✨ Features
- 🎬 Convert local MP4 files to high-quality MP3
- 📺 Download YouTube videos and convert to MP3
- 🚀 Built with Rust for maximum performance
- 🎯 Smart automatic file naming
- 📁 Automatic directory creation
- 🛡️ Comprehensive error handling

### 📦 Download
Choose the appropriate binary for your platform:

| Platform | Architecture | Download |
|----------|-------------|----------|
| 🍎 macOS | Apple Silicon (M1/M2) | [mp3-converter](https://github.com/sw3do/mp3-converter/releases/download/v0.1.0/mp3-converter) |
| 🪟 Windows | x64 | [mp3-converter.exe](https://github.com/sw3do/mp3-converter/releases/download/v0.1.0/mp3-converter.exe) |

### 🔧 Prerequisites
Before using, install:
- **FFmpeg** (for MP4 conversion)
- **yt-dlp** (for YouTube downloads)

See the [README](https://github.com/sw3do/mp3-converter#-installation) for detailed installation instructions.

### 📊 Technical Details
- **Binary Size:** ~1.4MB
- **Rust Version:** 1.75+
- **Audio Quality:** 192kbps, 44.1kHz MP3
- **Platforms:** macOS, Linux, Windows

### 🚀 Quick Start
```bash
# Convert MP4 to MP3
./mp3-converter convert -i video.mp4

# Download YouTube video as MP3
./mp3-converter download -u "https://www.youtube.com/watch?v=VIDEO_ID"
```

### 🔍 Verification
Verify your download with checksums:
```bash
shasum -a 256 mp3-converter-*
```

**Full Changelog**: https://github.com/sw3do/mp3-converter/commits/v0.1.0
```

5. Upload all files from `release-assets/` directory
6. Mark as "Latest release"
7. Publish the release

## 🔄 Future Releases

### Planned for v0.2.0
- 🎵 Additional audio format support (FLAC, AAC)
- 📁 Batch processing capabilities
- 🎛️ Custom audio quality settings
- 🔄 Progress bars for operations
- 🌐 Playlist download support
- 🏷️ Metadata preservation

### Release Schedule
- **v0.1.x**: Bug fixes and minor improvements
- **v0.2.0**: Feature expansion (Q2 2024)
- **v1.0.0**: Stable API and full feature set

## 📝 Release Checklist

### Pre-Release
- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Update `README.md` with new features
- [ ] Run full test suite
- [ ] Build all platform binaries
- [ ] Test binaries on target platforms

### Release
- [ ] Create git tag
- [ ] Push tag to GitHub
- [ ] Create GitHub release
- [ ] Upload all binary assets
- [ ] Generate and upload checksums
- [ ] Update release links in README

### Post-Release
- [ ] Announce on social media
- [ ] Update package managers (if applicable)
- [ ] Monitor for issues
- [ ] Plan next release features

## 🔐 Security

### Binary Verification
All release binaries include SHA256 checksums for verification:
```bash
# Verify download integrity
shasum -a 256 -c checksums.txt
```

### Signing (Future)
Future releases will include:
- GPG signatures for all binaries
- Code signing for macOS and Windows
- Supply chain security with SLSA

---

**Made with ❤️ by [sw3do](https://github.com/sw3do)**