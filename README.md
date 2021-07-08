# lfs

[![CI][s3]][l3] [![MIT][s2]][l2] [![Latest Version][s1]][l1] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/lfs.svg
[l1]: https://crates.io/crates/lfs

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://travis-ci.org/Canop/lfs.svg?branch=master
[l3]: https://travis-ci.org/Canop/lfs

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3768?Rust


A linux utility listing your filesystems.

![screenshot](doc/screenshot.png)

Besides traditional columns, the `disk` column helps you identify your "disk" (or the mapping standing between your filesystem and the physical device) :

* `remov` : a removable device (such as an USB key)
* `HDD` : a rotational disk
* `SSD` : a solid state storage device
* `RAM` : an in-memory device (such as zram)
* `LVM` : a device mapped to one or several disks using LVM
* `crypt` : a crypted disk

All sizes are based on the current SI recommendations: 1M is one million bytes.

## Installation

### Precompiled binary

You can download it from https://github.com/Canop/lfs/releases

### From source

You need the [Rust tool chain](https://rustup.rs/).

```
cargo install lfs
```

## Usage

```
lfs
```
By default, **lfs** only shows mount points backed by normal block devices, which are usually the "storage" filesystems you're interested into.

To show them all, use

```
lfs -a
```

To get the output as JSON, do `lfs -j` or  `lfs -a -j`.

You may pass a path to have only the relevant device shown.
For example:

![lfs dot](doc/lfs-dot.png)

Use `lfs --help` to list the other arguments.

## Internals

If you want to display the same data in your Rust application, have a look at the [lfs-core](https://docs.rs/lfs-core/) crate.
