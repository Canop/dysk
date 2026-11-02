
**dysk** works on Linux, Mac, and Windows.

Current version: **<a id=current-version href=../download>download</a>**
<script>
fetch("../download/version",{cache: "reload"})
    .then(response => response.text())
    .then(version => {
        version = version.trim();
        if (!/^\d+(\.\d+)*(-\w+)?$/.test(version)) {
            console.warn("invalid version in download/version");
            return;
        }
        document.getElementById("current-version").textContent = version;
    })
</script>

[CHANGELOG](https://github.com/Canop/dysk/blob/main/CHANGELOG.md)


# Precompiled binaries

Binaries are made available at every release in [download](https://dystroy.org/dysk/download).

The archives also contain dysk's man page and shell completion scripts.


[GitHub releases](https://github.com/Canop/dysk/releases) hold previous releases too, and, for each release, one archive per target (eg `dysk-v3.8.1-x86_64-unknown-linux-musl.tar.gz`) with a `sha256sums` file.

When you download executable files, you'll have to ensure the shell can find them. An easy solution on linux is for example to put them in `/usr/local/bin`. You may also have to set them executable using `chmod +x dysk`.

For a typical Unix installation (Linux or macOS), copy the extracted files to destinations such as these:

| File in the archive | Example destination | Purpose |
|---|---|---|
| `dysk` | `/usr/local/bin/dysk` | Executable |
| `man/dysk.1` | `/usr/local/share/man/man1/dysk.1` | Man page (`man dysk`) |
| `completion/dysk.bash` | `/usr/local/share/bash-completion/completions/dysk` | Optional Bash completion |
| `completion/_dysk` | `/usr/local/share/zsh/site-functions/_dysk` | Optional Zsh completion |
| `completion/dysk.fish` | `~/.config/fish/completions/dysk.fish` | Optional Fish completion |
| `completion/dysk.elv` | `~/.config/elvish/lib/dysk.elv` | Optional Elvish completion module |

The paths in the first column are relative to the extracted `dysk-v<version>-<triple>/` directory of a per-target archive. In the all-target `dysk_<version>.zip`, choose the executable in the directory for your target (for example `aarch64-apple-darwin/dysk`); `man/` and `completion/` are at the archive root.

These are examples, not Windows installation paths. The `/usr/local` destinations are system-wide and may require administrator permissions; `~` refers to your home directory. Create missing directories as needed, and adjust the prefix and lookup paths for your system's executable, man-page, and shell configuration.

Only install the optional completion for the shell you use:

- Bash needs bash-completion installed and loaded. Copy `dysk.bash` into its completion directory with the installed name `dysk`; the directory may differ from the example above.
- Zsh needs the directory containing `_dysk` on `fpath` before its completion system is initialized with `autoload -Uz compinit; compinit` in `~/.zshrc` (unless your shell configuration already initializes it).
- Fish automatically loads `dysk.fish` from its user completion directory; adjust the example if you use a different Fish configuration directory.
- Elvish needs the module loaded: add `use dysk` to `~/.config/elvish/rc.elv`, adjusting both Elvish paths if you use a different configuration directory.

# From crates.io

You'll need to have the [Rust development environment](https://www.rustup.rs) installed and up to date.

Once it's installed, use cargo to install dysk:

    cargo install --locked dysk

**Note:**
If there's a compilation error, it most often means either that you're missing some compilation dependency (on ubuntu/debian try `sudo apt install build-essential`) or that you have an old version of the compiler, and you should update it (for example with `rustup update`).

# From source

You'll need to have the [Rust development environment](https://www.rustup.rs) installed.

Fetch the [Canop/dysk](https://github.com/Canop/dysk) repository, move to the dysk directory, then run

```bash
cargo install --locked --path .
```

# Community maintained

## conda-forge

dysk is available as a conda package [on conda-forge](https://github.com/conda-forge/dysk-feedstock) for the following platforms

[![Conda Platforms](https://img.shields.io/conda/pn/conda-forge/dysk.svg)](https://anaconda.org/conda-forge/dysk)

and can be installed globally with [Pixi](https://pixi.sh/):

    pixi global install dysk

or into a particular project with Pixi:

    pixi add dysk

or with [conda](https://docs.conda.io/projects/conda/):

    conda install --channel conda-forge dysk

## Other repositories

Those packages are maintained by third parties and may be less up to date.

[![Packaging status](https://repology.org/badge/vertical-allrepos/dysk.svg)](https://repology.org/project/dysk/versions)

