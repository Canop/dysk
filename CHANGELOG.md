<a name="v2.3.0"></a>
### v2.3.0 - 2022/02/27
- "remote" column. Remote filesystems included by default - Fix #33

<a name="v2.2.0"></a>
### v2.2.0 - 2022/02/26
- `--sort` launch argument for sorting rows in table - Fix #37

<a name="v2.1.1"></a>
### v2.1.1 - 2022/02/25
- `--list-cols` launch argument for knowing the columns and their names

<a name="v2.1.0"></a>
### v2.1.0 - 2022/02/23
- fix failure in parsing `--cols` arguments with underscore
- list all column names in case of bad value of `--cols`
- improve alignement of the 'inodes' column - Fix #38
- it's now possible to have size info but no inodes, so as to be compatible with filesystems not filling inodes info in a consistent way - Fix #36
- breaking change: inodes related fields in the JSON have been moved to a dedicated `inodes` struct (because they're now optional)
- addition of 2 new non default columns: `use_percent` and `inodes_percent`
- switch columns type and disk: it makes more sense to have the type of filesystem just after the filesystem name

<a name="v2.0.2"></a>
### v2.0.2 - 2022/02/23
- show ZFS volumes among "normal" filesystems, even when the disk can't be determined - Fix #32

<a name="v2.0.1"></a>
### v2.0.1 - 2022/02/20
- align filesystem column to the left

<a name="v2.0.0"></a>
### v2.0.0 - 2022/02/20
- It's now possible to set the columns with the `--cols` launch argument
- default column set has changed
- `--inodes` and `--label` have been removed (example: to see labels, use `lfs -c +label`)

<a name="v1.4.0"></a>
### v1.4.0 - 2022/01/06
- bound mounts hidden by default

<a name="v1.3.1"></a>
### v1.3.1 - 2021/12/25
- upgrade termimad for better table fitting (especially when some mount points have long paths)

<a name="v1.3.0"></a>
### v1.3.0 - 2021/11/03
- inodes stats (total, free, used, % used) added to JSON
- `--inodes` (or `-i`) launch argument adds a "inodes use" column to the table - Fix #23

<a name="v1.2.1"></a>
### v1.2.1 - 2021/10/30
- decode ascii-hexa encoded labels (i.e. displays "/home" instead of "\x2fhome")

<a name="v1.2.0"></a>
### v1.2.0 - 2021/10/16
- filesystem labels added to JSON when found
- `--labels` (`-l` in short) launch argument adds a "label" column to the table

<a name="v1.1.0"></a>
### v1.1.0 - 2021/10/08
--units launch argument, to choose between SI units or the old binary ones - Fix #17

<a name="v1.0.0"></a>
### v1.0.0 - 2021/09/05
I see no reason not to tag this a 1.0

<a name="v0.7.6"></a>
### v0.7.6 - 2021/07/08
* better identify mapped devices (such as LVM)

<a name="v0.7.5"></a>
### v0.7.5 - 2021/07/01
* fix endless loops in some configurations - Fix #13

<a name="v0.7.4"></a>
### v0.7.4 - 2021/07/01
* `--color` option with values yes|no|auto (auto being default)
* no tty style when `--color` is default and the output is piped

<a name="v0.7.3"></a>
### v0.7.3 - 2021/06/30
* fix disk not found for BTRFS filesystems - Fix #11

<a name="v0.7.2"></a>
### v0.7.2 - 2021/06/29
* use termimad 0.13 for better support of narrow terminals and wide chars

<a name="v0.7.1"></a>
### v0.7.1 - 2021/06/24
* better column balancing in table display

<a name="v0.7.0"></a>
### v0.7.0 - 2021/06/23
* use bars to better display disk use
* you may pass a path as argument to have lfs show only the relevant device

<a name="v0.6.0"></a>
### v0.6.0 - 2021/06/22
* tag zram "disks" as "RAM"
* list and identify crypted disks

<a name="v0.5.4"></a>
### v0.5.4 - 2021/06/21
* fix missing size of disk whose name contains a space character

<a name="v0.5.3"></a>
### v0.5.3 - 2020/10/18
* now compiles on 32 bits platforms too (but tests lacking)

<a name="v0.5.2"></a>
### v0.5.2 - 2020/10/17
* `--json` option to output the data in JSON

<a name="v0.5.1"></a>
### v0.5.1 - 2020/10/16
* `--version`

<a name="v0.5.0"></a>
### v0.5.0 - 2020/10/15
* identifies removable devices as such

<a name="v0.4.0"></a>
### v0.4.0 - 2020/10/13
* Based on a new version of lfs-core, this version better identifies disk types.
* By default, only filesystems backed by a block devices are shown now

<a name="v0.3.0"></a>
### v0.3.0 - 2020/10/12
First "public" version, not really tested


