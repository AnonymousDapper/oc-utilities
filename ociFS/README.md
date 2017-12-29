
# ociFS On-Disk Format

All data is stored on disk as little-endian.

## Boot block

If the boot data doesn't take up the entire block, the rest of the block will be assumed empty, but no reading or writing will be done in that space.

| Offset | Length   | Description                               |
| ------ | -------- | ----------------------------------------- |
| 0x00   | 6 bytes  | Filesystem driver label `ociFS\xFF`       |
| 0x06   | 1 byte   | Filesystem driver major version           |
| 0x07   | 1 byte   | Filesystem driver minor version           |
| 0x08   | 2 bytes  | Number of bytes per block                 |
| 0x0A   | 2 bytes  | Number of blocks on disk                  |
| 0x0C   | 2 bytes  | Number of blocks allocated for file units |
| 0x0E   | 2 bytes  | Boot block signature `\x5F\xF5`           |


On a typical Tier 3 disk, there will be 256 bytes per block, 16,384 blocks on the disk and 1,434 blocks reserved for allocation units.

This allows for roughly 358 Kb of allocation data, and just over 3.6 Mb of filesystem data.


## FAT blocks

The FAT stores information about the current state of all the file entries on the disk.

All object IDs are 24 bits in length.

| Offset | Length   | Description                               |
| ------ | -------- | ----------------------------------------- |
| 0x00   | 3 byes   | Number of allocation units on disk        |
| 0x03   | 3 bytes  | Address of first allocation unit          |
| 0x06   | 1 byte   | Disk read-only flag (0 or 1)              |
| 0x07   | 1 byte   | FAT block signature `\xFA`                |


The first allocation unit on disk always starts in the second block directly after the FAT, and additional units start directly after the previous. Each allocation unit is exactly 36 bytes long.

If the final unit does not end at a block boundary, the rest of that block will be zero-padded and filesystem data will be stored on the block following it.

| Offset | Length   | Description                                 |
| ------ | -------- | ------------------------------------------- |
| 0x00   | 1 byte   | Object type indicator (0 = File, 1 = Dir)   |
| 0x01   | 3 bytes  | Object ID (see below)                       |
| 0x04   | 33 bytes | Object name (see below)                     |
| 0x25   | 7 bytes  | Object file extension - dot is implied      |
| 0x2C   | 1 byte   | Object permissions (NYI)                    |
| 0x2D   | 2 bytes  | Object owner ID (NYI)                       |
| 0x2F   | 2 bytes  | Object group ID (NYI)                       |
| 0x31   | 2 bytes  | Object creation time (see below)            |
| 0x33   | 2 bytes  | Object creation date (see below)            |
| 0x35   | 3 bytes  | Object reference location (see below)       |
| 0x38   | 3 bytes  | Object file size in bytes, 0 if a directory |


* Object ID: `0x00000` to `0xFFFFFE` are usable ids. `0xFFFFFE` means the object has been deleted and `0xFFFFFF` means the object is the root directory.


* Object Name: The name string is a zero-terminated, zero-padded string with max length of 32 bytes. The encoding is assumed to be ASCII.


* Object Creation Time: `hhhhh` indicates the number of hours (0-23), `mmmmmm` indicates the number of minutes (0-59), `xxxxx` indicates the number of two-second intervals (0-29).
  ```
  <------- 0x1B --------> <------- 0x1A -------->
  15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00
  h  h  h  h  h  m  m  m  m  m  m  x  x  x  x  x
  ```


* Object Creation Date: `yyyyyyy` indicates the year offset from 2000 (0-127), `mmmm` indicates the month number (1-12), `ddddd` indicates the day number (1-31).
  ```
  <------- 0x1D --------> <------- 0x1C -------->
  15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00
  y  y  y  y  y  y  y  m  m  m  m  d  d  d  d  d
  ```


* Object Reference: If object is a file, is an address pointing to the block starting the file data. If a directory, is the parent directory's ID. For the root directory, this is `0xFFFFFF`.

# NOTICE

This is not complete, it does not have a way for files to inherit a parent directory.

# NOTICE