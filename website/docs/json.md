
With `lfs --json` (shortened in `lfs -j`) you get a JSON structure that can be used in other programs.

# Raw output

Here's a sample, that I could get with `lfs -j > disks.json`:

```JSON
[
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
    "stats": {
      "available": "82G",
      "bavail": 20034649,
      "bfree": 23060978,
      "blocks": 59233748,
      "bsize": 4096,
      "favail": 13907592,
      "ffree": 13907592,
      "files": 15114240,
      "size": "243G",
      "used": "161G",
      "used-percent": "66%"
    }
  },
  {
    "bound": false,
    "dev": {
      "major": 8,
      "minor": 17
    },
    "disk": {
      "crypted": false,
      "ram": false,
      "removable": false,
      "rotational": false,
      "type": "SSD"
    },
    "fs": "/dev/sdb1",
    "fs-label": null,
    "fs-type": "xfs",
    "id": 106,
    "mount-point": "/home/dys/dev",
    "stats": {
      "available": "561G",
      "bavail": 136864893,
      "bfree": 136864893,
      "blocks": 244071157,
      "bsize": 4096,
      "favail": 487302056,
      "ffree": 487302056,
      "files": 488380736,
      "size": "1.0T",
      "used": "439G",
      "used-percent": "44%"
    }
  }
]
```

# All filesystems

As for the table view, the JSON is by default limited to "normal" storage devices.

You can get the complete list with `lfs --json --all`.

# Pipe

The output of `lfs -j` can be piped into another program.

You can for example use [jq](https://stedolan.github.io/jq/) to filter or extract data:

![jq](img/json-jq.png)

