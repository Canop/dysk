
**dysk** works on Linux, Mac, and Windows (experimental).

Current version: **<a id=current-version href=../download>download</a>**
<script>
console.log("in script");
fetch("../download/version",{cache: "reload"})
    .then(response => response.text())
    .then(version => {
        console.log(`version: #${version}#`);
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


You may download previous releases on [GitHub releases](https://github.com/Canop/dysk/releases).

When you download executable files, you'll have to ensure the shell can find them. An easy solution on linux is for example to put them in `/usr/local/bin`. You may also have to set them executable using `chmod +x dysk`.

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

