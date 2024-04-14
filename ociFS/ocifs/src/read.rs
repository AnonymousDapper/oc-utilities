// The MIT License (MIT)

// Copyright (c) 2024 AnonymousDapper

use crate::{SuperBlock, Validate, MAGIC, ROOT_BLOCK_OFFSET};

use std::io::{self, BufReader, Read, Seek};

use bytemuck::Pod;
use std::mem::{self, MaybeUninit};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("device read failed")]
    IOError(#[from] io::Error),
    #[error("invalid signature")]
    BadMagic,
    #[error("invalid data")]
    Invalid,
    #[error("something else, idk")]
    Unknown,
}

fn read_object<T, R>(reader: &mut R) -> Result<T, ReadError>
where
    T: Pod + Validate,
    R: Read,
{
    let mut uninit: MaybeUninit<T> = MaybeUninit::zeroed();
    let buf = unsafe {
        std::slice::from_raw_parts_mut(uninit.as_mut_ptr() as *mut u8, mem::size_of::<T>())
    };

    reader.read_exact(buf)?;

    let obj: T = unsafe { uninit.assume_init() };

    if obj.is_valid() {
        Ok(obj)
    } else {
        println!("{buf:#04X?}");
        Err(ReadError::Invalid)
    }
}

pub struct BlockReader<R: Read + Seek> {
    reader: BufReader<R>,
}

impl<R> BlockReader<R>
where
    R: Read + Seek,
{
    pub fn new(handle: R) -> Self {
        Self {
            reader: BufReader::new(handle),
        }
    }

    pub fn get_vol_size(&mut self) -> Result<u64, ReadError> {
        let old = self.reader.stream_position()?;
        self.reader.seek(io::SeekFrom::End(0))?;
        let size = self.reader.stream_position()?;
        self.reader.seek(io::SeekFrom::Start(old))?;
        Ok(size)
    }

    pub fn read_superblock(&mut self) -> Result<SuperBlock, ReadError> {
        self.reader
            .seek(io::SeekFrom::Start(ROOT_BLOCK_OFFSET.into()))?;

        let super_block: SuperBlock = read_object(&mut self.reader)?;

        if &super_block.magic == MAGIC {
            Ok(super_block)
        } else {
            Err(ReadError::BadMagic)
        }
    }
}
