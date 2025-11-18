# Build from Source

Building from source gives you the latest version and allows you to customize the build:

```bash
git clone https://github.com/haruki7049/kakei.git
cd kakei
cargo build --release
```

The binary will be available at `target/release/kakei`. You can copy it to a directory on your PATH (e.g., `/usr/local/bin`) if desired:

```bash
sudo cp target/release/kakei /usr/local/bin/
```
