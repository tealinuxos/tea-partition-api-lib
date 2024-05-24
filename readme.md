# Parted Parser

using parted with flag machine parseable output, we able to get disk and its partitions.

# Structs

```rust
#[derive(Debug)]
struct Disk {
    path: String,
    size: String,
    model: String,
    transport: String,
    logical_sector_size: usize,
    physical_sector_size: usize,
    label: String,
    partitions: Vec<Partition>,
}

#[derive(Debug)]
struct Partition {
    number: usize,
    start: String,
    end: String,
    size: String,
    name: String,
    filesystem: String,
    flags: Vec<String>,
}
```

# Example

```bash
$ sudo parted -l
Model: SanDisk Cruzer Blade (scsi)
Disk /dev/sda: 15.4GB
Sector size (logical/physical): 512B/512B
Partition Table: msdos
Disk Flags: 

Number  Start  End     Size   Type     File system  Flags
 2      979MB  1130MB  151MB  primary  fat16        esp


Model: INTEL SSDPEKNU512GZ (nvme)
Disk /dev/nvme0n1: 512GB
Sector size (logical/physical): 512B/512B
Partition Table: gpt
Disk Flags: 

Number  Start   End    Size    File system     Name                          Flags
 1      1049kB  274MB  273MB   fat32           EF                            boot, esp
 2      274MB   290MB  16.8MB                  Microsoft reserved partition  msftres, no_automount
 3      290MB   147GB  147GB   ntfs            Basic data partition          msftdata
 4      166GB   173GB  6300MB  ext4
 5      173GB   216GB  42.9GB  ext4
 6      216GB   383GB  167GB   ext4
 8      383GB   490GB  107GB   ext4
 9      490GB   497GB  7516MB  linux-swap(v1)                                swap


$ sudo parted -l
BYT;
/dev/sda:15.4GB:scsi:512:512:msdos:SanDisk Cruzer Blade:;
2:979MB:1130MB:151MB:fat16::esp;

BYT;
/dev/nvme0n1:512GB:nvme:512:512:gpt:INTEL SSDPEKNU512GZ:;
1:1049kB:274MB:273MB:fat32:EF:boot, esp;
2:274MB:290MB:16.8MB::Microsoft reserved partition:msftres, no_automount;
3:290MB:147GB:147GB:ntfs:Basic data partition:msftdata;
4:166GB:173GB:6300MB:ext4::;
5:173GB:216GB:42.9GB:ext4::;
6:216GB:383GB:167GB:ext4::;
8:383GB:490GB:107GB:ext4::;
9:490GB:497GB:7516MB:linux-swap(v1)::swap;
```

rust output:

```bash
$ sudo -E cargo r
[
    Disk {
        path: "/dev/sda",
        size: "15.4GB",
        model: "SanDisk Cruzer Blade",
        transport: "scsi",
        logical_sector_size: 512,
        physical_sector_size: 512,
        label: "msdos",
        partitions: [
            Partition {
                number: 2,
                start: "979MB",
                end: "1130MB",
                size: "151MB",
                name: "",
                filesystem: "fat16",
                flags: [
                    "esp",
                ],
            },
        ],
    },
    Disk {
        path: "/dev/nvme0n1",
        size: "512GB",
        model: "INTEL SSDPEKNU512GZ",
        transport: "nvme",
        logical_sector_size: 512,
        physical_sector_size: 512,
        label: "gpt",
        partitions: [
            Partition {
                number: 1,
                start: "1049kB",
                end: "274MB",
                size: "273MB",
                name: "EF",
                filesystem: "fat32",
                flags: [
                    "boot",
                    "esp",
                ],
            },
            Partition {
                number: 2,
                start: "274MB",
                end: "290MB",
                size: "16.8MB",
                name: "Microsoft reserved partition",
                filesystem: "",
                flags: [
                    "msftres",
                    "no_automount",
                ],
            },
            Partition {
                number: 3,
                start: "290MB",
                end: "147GB",
                size: "147GB",
                name: "Basic data partition",
                filesystem: "ntfs",
                flags: [
                    "msftdata",
                ],
            },
            Partition {
                number: 4,
                start: "166GB",
                end: "173GB",
                size: "6300MB",
                name: "",
                filesystem: "ext4",
                flags: [],
            },
            Partition {
                number: 5,
                start: "173GB",
                end: "216GB",
                size: "42.9GB",
                name: "",
                filesystem: "ext4",
                flags: [],
            },
            Partition {
                number: 6,
                start: "216GB",
                end: "383GB",
                size: "167GB",
                name: "",
                filesystem: "ext4",
                flags: [],
            },
            Partition {
                number: 8,
                start: "383GB",
                end: "490GB",
                size: "107GB",
                name: "",
                filesystem: "ext4",
                flags: [],
            },
            Partition {
                number: 9,
                start: "490GB",
                end: "497GB",
                size: "7516MB",
                name: "",
                filesystem: "linux-swap(v1)",
                flags: [
                    "swap",
                ],
            },
        ],
    },
]
```
