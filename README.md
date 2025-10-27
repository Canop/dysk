# dysk

[![CI][s3]][l3] [![MIT][s2]][l2] [![Latest Version][s1]][l1] [![Chat on Miaou][s4]][l4] [![Packaging status][srep]][lrep]

[s1]: https://img.shields.io/crates/v/dysk.svg
[l1]: https://crates.io/crates/dysk

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://travis-ci.org/Canop/dysk.svg?branch=master
[l3]: https://travis-ci.org/Canop/dysk

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/4847?dysk

[srep]: https://repology.org/badge/tiny-repos/dysk.svg
[lrep]: https://repology.org/project/dysk/versions

A Linux/Mac utility listing your filesystems.

Complete documentation lives at **[https://dystroy.org/dysk](https://dystroy.org/dysk)**

* **[Overview](https://dystroy.org/dysk/)**
* **[Installation](https://dystroy.org/dysk/install)**

dysk was previously known as lfs.

### Default table

![screenshot](website/docs/img/dysk.png)

### Custom choice of column

![screenshot](website/docs/img/dysk_c=label+default+dev.png)

![screenshot](website/docs/img/dysk_c=+dev+inodes.png)

### JSON output

![screenshot](website/docs/img/dysk-json-jq.png)

You can output the table as CSV too.

### Filters

![screenshot](website/docs/img/dysk_filters.png)

### Sort

![screenshot](website/docs/img/dysk_s=free-d.png)

### Library

The data displayed by dysk is provided by the [lfs-core](https://github.com/Canop/lfs-core) crate.
You may use it in your own Rust application.

