
# ocFAT On-Disk Format

## Boot block

If the boot data doesn't take up the entire block, the rest of the block will be assumed empty, but no reading or writing will be done in that space.

| Offset | Length   | Description                               |
| ------ | -------- | ----------------------------------------- |
| 0x00   | 6 bytes  | Filesystem driver label `ocFAT\xFF`       |
| 0x06   | 1 byte   | Filesystem driver major version           |
| 0x07   | 1 byte   | Filesystem driver minor version           |
| 0x08   | 2 bytes  | Number of bytes per block                 |
| 0x0A   | 2 bytes  | Number of blocks on disk                  |
| 0x0C   | 2 bytes  | Number of FATs on disk                    |
| 0x0E   | 2 bytes  | Boot block signature `\x5F\xF5`           |

On a typical disk, there will be 256 bytes per block, 16,384 blocks on the disk and 2,000 file allocation entries on the disk.
This leaves 30,000 bytes worth of FAT data, 313 blocks used for FAT, 1 block used for boot data, and 16,070 blocks left for filesysem data.
This leaves 3.9Mb of filesystem data available


## FAT section

The first FAT on disk always starts at the second block, and additional FATs start at the index specified by the previous FAT.
If the final FAT does not end at a block boundary, the rest of that block will be zero-padded and filesystem data will be stored on the block following it.

24 bit ID system allows for 16777213 unique objects.

| Offset | Length   | Description                                 |
| ------ | -------- | ------------------------------------------- |
| 0x00   | 1 byte   | Object type indicator (0 = File, 1 = Dir)   |
| 0x01   | 3 bytes  | Object ID (see below)                       |
| 0x04   | 17 bytes | Object name (see below)                     |
| 0x15   | 1 byte   | Object permissions (NYI)                    |
| 0x16   | 2 byte   | Object owner ID (NYI)                       |
| 0x18   | 2 byte   | Object group ID (NYI)                       |
| 0x1A   | 1 byte   | Object creation time seconds                |
| 0x1B   | 1 byte   | Object creation time minutes                |
| 0x1C   | 1 byte   | Object creation time hours                  |
| 0x1D   | 1 byte   | Object creation time day of month           |
| 0x1E   | 1 byte   | Object creation time month                  |
| 0x1F   | 2 bytes  | Object creation time year                   |
| 0x20   | 3 bytes  | Object reference location (see below)       |
| 0x23   | 3 bytes  | Object file size in bytes, 0 if a directory |
| 0x25   | 1 byte   | FAT block signature `\xFA`                  |

Object ID: `0x00000` to `0xFFFFFE` are usable ids. `0xFFFFFE` means the object has been deleted and `0xFFFFFF` means the object is the root directory

Object Name: The name string is a zero-terminated, zero-padded ASCII string with max length of 16 bytes.

Object Reference: If a file, is an address pointing to the block starting the file data. If a directory, is the parent directory's ID

Note: Each FAT block is 40 bytes long. (Lua pack string `<BI3c17BI2I2BBBBBI2I3I3B`)

