
**lfs** only works on linux.

# Precompiled binaries

Binaries are made available at every release in [download](https://dystroy.org/lfs/download).

Direct links:

Target|Files
-|-
Android | [aarch64-linux-android](https://dystroy.org/lfs/download/aarch64-linux-android/lfs)
Linux | [x86_64-linux](https://dystroy.org/lfs/download/x86_64-linux/lfs)
Linux/musl | [x86_64-unknown-linux-musl](https://dystroy.org/lfs/download/x86_64-unknown-linux-musl/lfs)
Raspberry | [armv7-unknown-linux-gnueabihf](https://dystroy.org/lfs/download/armv7-unknown-linux-gnueabihf/lfs)

You may download previous releases on [GitHub releases](https://github.com/Canop/lfs/releases).

When you download executable files, you'll have to ensure the shell can find them. An easy solution is for example to put them in `/usr/local/bin`. You may also have to set them executable using `chmod +x lfs`.

# From crates.io

You'll need to have the [Rust development environment](https://www.rustup.rs) installed and up to date.

Once it's installed, use cargo to install lfs:

    cargo install lfs

# From source

You'll need to have the [Rust development environment](https://www.rustup.rs) installed.

Fetch the [Canop/lfs](https://github.com/Canop/lfs) repository, move to the lfs directory, then run

```bash
cargo install --path .
```

!!! Note
	If there's a compilation error, it most often means either that you're missing some compilation dependency (on ubuntu/debian try `sudo apt install build-essential`) or that you have an old version of the compiler, and you should update it (for example with `rustup update`).

# Third party repositories

Those packages are maintained by third parties and may be less up to date.

## Arch Linux

**lfs** can be installed from the [community repository](https://archlinux.org/packages/community/x86_64/lfs/):

```
pacman -S lfs
```

