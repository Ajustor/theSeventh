# The Seventh

A 2D platformer game built with Bevy and Rust.

## Building the Game

### Prerequisites

- Rust (latest stable version)
- Cargo

### Development Build

```bash
cargo build
cargo run
```

### Release Build

```bash
cargo build --release
```

The release binary will be located at `target/release/theSeventh` (or `theSeventh.exe` on Windows).

## Asset Management

The game requires assets to be present alongside the executable. The `assets/` folder contains:

- **Images**: Player sprites, atlas textures, and game graphics
- **Level files**: LDtk level definitions (`.ldtk` files)

### Running the Game

When running the game, ensure the `assets/` folder is in the same directory as the executable:

```
theSeventh/
├── theSeventh (or theSeventh.exe)
└── assets/
    ├── Typical_2D_platformer_example.ldtk
    ├── player.png
    └── atlas/
        └── *.png
```

### Distribution

Release artifacts automatically include the assets folder. The GitHub Actions release workflow:

1. Builds the game in release mode
2. Creates a `release/` directory
3. Copies the executable and assets folder
4. Creates platform-specific archives (`.tar.gz` for Unix, `.zip` for Windows)
5. Verifies that assets are present in the final archive

To create a release:

```bash
git tag v0.x.x
git push origin v0.x.x
```

This will trigger the release workflow which builds for:
- Windows (x64)
- Linux (x64)
- macOS (x64 and ARM64)

## Platform-Specific Notes

### Linux Dependencies

On Linux, you need to install some system dependencies:

```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
```

### Running with Release Optimizations

The game should be run with release optimizations for best performance:

```bash
cargo run --release
```
