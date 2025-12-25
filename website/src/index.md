<p class=logo>
<img class=logo width=300px src="img/dysk-homard-logo.svg">
</p>

# Purpose

**dysk** is a Linux/Mac/Windows utility listing your filesystems.

![standard](img/dysk.png)

Usage is shown in a graphical way.

Besides traditional columns, the `disk` column helps you identify your "disk" (or the mapping standing between your filesystem and the physical device) :

* `remov` : a removable device (such as an USB key)
* `HDD` : a rotational disk
* `SSD` : a solid state storage device
* `RAM` : an in-memory device (such as zram)
* `LVM` : a device mapped to one or several disks using LVM
* `crypt` : a crypted disk

By default, only the "normal" devices, the ones you're usually interested about, are shown, but you can see the other ones with the `-a` option.

All sizes are normally based on the current SI recommendations (1M is one million bytes) but can be changed with `--units binary` (then 1M is 1,048,576 bytes).

# Tables

The default display of **dysk** is a table, which can be configured with the columns of your choice.

![screen](img/dysk_c=label+.png)

See [Table](./table) for the definition of the columns and the syntax for choosing them, or on how to [sort rows](./table#sort).

The table can also be exported in [CSV](./table#csv).

# JSON

`dysk --json` outputs the result as JSON which can be used for your own scripts or programs.

![screen](img/dysk-json-jq.png)

# Filters

The query syntax lets you specify the filesystems you want:

![screen](img/dysk_filters.png)
