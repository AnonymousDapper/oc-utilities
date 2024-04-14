
# ociFS On-Disk Format

ociFS uses a simplified implementation of the Ext2 file system.

All data is stored on disk as little-endian.

Pointers and addresses are 32-bit.

The first 128 bytes of the partition are reserved, and the root superblock is at 0x80.

A block group is composed of a:
- Superblock (sometimes)
- Group Descriptor blocks (sometimes)
- Data bitmap
- Inode bitmap
- Inode tables
- Data blocks

## Superblock

The superblock is 128 bytes, but the lower half are reserved for future use.

| Start  | Length | Description                                 |
| ------ | ------ | ------------------------------------------- |
| 0      | 4      | ociFS signature "ocFS" 0x6F 63 46 53        |
| 4      | 1      | Major version                               |
| 5      | 1      | Minor version                               |
| 6      | 2      | Total number of inodes                      |
| 8      | 2      | Total number of blocks                      |
| 10     | 2      | Total unallocated inodes                    |
| 12     | 2      | Total unallocated blocks                    |
| 14     | 2      | Number of inodes per block group            |
| 16     | 2      | Number of blocks per block group            |
| 18     | 2      | Reserved                                    |
| 20     | 16     | Filesystem UUID                             |
| 36     | 24     | Volume name (C string)                      |
| 60     | 4      | log_2 (block size) - 10                     |
| 64     | 64     | Padding (future reserved)                   |


## Group Descriptor

| Start | Length | Description                                  |
| ----- |------- | -------------------------------------------- |
| 0     | 4      | Block address of the data bitmap             |
| 4     | 4      | Block address of the inode bitmap            |
| 8     | 4      | Block address of the inode table             |
| 12    | 2      | Number of unallocated blocks in the group    |
| 14    | 2      | Number of unallocated inodes in the group    |
| 16    | 2      | Number of directories in the group           |
| 18    | 14     | Unused                                       |