// The MIT License (MIT)

// Copyright (c) 2024 AnonymousDapper

use bytemuck::{Pod, Zeroable};
use std::ffi::{CStr, CString};
use std::fmt;
use type_layout::TypeLayout;

pub mod read;
pub mod write;

pub const MAGIC: &[u8; 4] = b"ocFS";
pub const ROOT_BLOCK_OFFSET: u32 = 0x80;

pub type Padding<const N: usize> = [u8; N];
pub type Pointer = u32;

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod, TypeLayout)]
pub struct UUID {
    time_low: u32,
    time_middle: u16,
    time_high_version: u16,
    sequence_reserved: u8,
    sequence_low: u8,
    node: [u8; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod, TypeLayout)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod, TypeLayout)]
pub struct SuperBlock {
    magic: [u8; 4],
    pub version: Version,
    pub total_inodes: u16,
    pub total_blocks: u16,
    pub unalloc_inodes: u16,
    pub unalloc_blocks: u16,
    pub group_inodes: u16,
    pub group_blocks: u16,
    pub reserved: u16,
    pub fs_id: UUID,
    volume_name: [u8; 24],
    log_block_size: u32,
    pad: Padding<64>,
}

impl SuperBlock {
    pub fn volume_name(&self) -> Option<&str> {
        let c_str = CStr::from_bytes_until_nul(&self.volume_name);
        match c_str {
            Ok(c_str) => c_str.to_str().ok(),
            Err(_) => None,
        }
    }

    pub fn rename(&mut self, name: &str) {
        let c_str = CString::new(name);
        match c_str {
            Ok(c_str) => {
                let bytes = c_str.as_bytes_with_nul();
                self.volume_name = [0; 24];
                self.volume_name[..bytes.len()].copy_from_slice(bytes);
            }
            Err(e) => eprintln!("Failed to rename volume: {}", e),
        }
    }

    pub fn block_size(&self) -> u32 {
        1024 << self.log_block_size
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod, TypeLayout)]
pub struct GroupDescriptor {
    data_bitmap: Pointer,
    inode_bitmap: Pointer,
    inode_table: Pointer,
    unalloc_blocks: u16,
    unalloc_inodes: u16,
    dir_count: u16,
    pad: Padding<14>,
}

// 28aa3602-7379-4675-a6f2-7873e963f3f0

pub trait Validate {
    fn is_valid(&self) -> bool;
}

impl Validate for UUID {
    fn is_valid(&self) -> bool {
        self.time_low != 0
            && self.time_middle != 0
            && self.time_high_version != 0
            && self.sequence_reserved != 0
            && self.sequence_low != 0
            && self.node.iter().all(|&x| x != 0)
    }
}

impl Validate for Version {
    fn is_valid(&self) -> bool {
        true
    }
}

impl Validate for SuperBlock {
    fn is_valid(&self) -> bool {
        self.version.is_valid()
            && self.total_inodes != 0
            && self.total_blocks != 0
            && self.group_inodes != 0
            && self.group_blocks != 0
            && self.fs_id.is_valid()
    }
}

impl Validate for GroupDescriptor {
    fn is_valid(&self) -> bool {
        self.data_bitmap != 0 && self.inode_bitmap != 0 && self.inode_table != 0
    }
}

impl fmt::Display for UUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.time_low,
            self.time_middle,
            self.time_high_version,
            self.sequence_reserved,
            self.sequence_low,
            self.node[0],
            self.node[1],
            self.node[2],
            self.node[3],
            self.node[4],
            self.node[5]
        )
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}
