
With `dysk --json` (shortened in `dysk -j`) you get a JSON structure that can be used in other programs.

# JSON output

The normal output is an array of all filesystems matching the filter.

In order to make a sample fitting this site, I extracted the first filesystem with

```bash
dysk -j | jq '.[0]' > disk.json
```

Here it is:

```JSON
{
  "bound": false,
  "dev": {
    "major": 8,
    "minor": 1
  },
  "disk": {
    "crypted": false,
    "ram": false,
    "removable": false,
    "rotational": false,
    "type": "SSD"
  },
  "fs": "/dev/sda1",
  "fs-label": null,
  "fs-type": "ext4",
  "id": 26,
  "mount-point": "/",
  "remote": false,
  "stats": {
    "available": "81G",
    "bavail": 19764035,
    "bfree": 22790364,
    "blocks": 59233748,
    "bsize": 4096,
    "inodes": {
      "avail": 13880393,
      "files": 15114240,
      "free": 13880393,
      "used-percent": "8%"
    },
    "size": "243G",
    "used": "162G",
    "used-percent": "67%"
  },
  "unreachable": false
}
```

The `disk`, `stats`, and `stats.inodes` structures, or the `fs-label`, may be `null` for some filesystems.

Note that fields may be *added* in any version.

# All filesystems

As for the table view, the JSON is by default limited to "normal" storage devices.

You can get the complete list with `dysk --json --all`.

# Pipe

The output of `dysk -j` can be piped into another program.

You can for example use [jq](https://stedolan.github.io/jq/) to filter or extract data:

![jq](img/dysk-json-jq.png)

