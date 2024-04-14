// The MIT License (MIT)

// Copyright (c) 2024 AnonymousDapper

use crate::{SuperBlock, ROOT_BLOCK_OFFSET};

use std::io::{self, BufWriter, Seek, Write};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("device write failed")]
    IOError(#[from] io::Error),
    #[error("something else, idk")]
    Unknown,
}

// fn read_object<T, R>(reader: &mut R) -> Result<T, ReadError>
// where
//     T: Pod,
//     R: Read,
// {
//     let mut uninit: MaybeUninit<T> = MaybeUninit::zeroed();
//     let buf = unsafe {
//         std::slice::from_raw_parts_mut(uninit.as_mut_ptr() as *mut u8, mem::size_of::<T>())
//     };

//     reader.read_exact(buf)?;

//     unsafe { Ok(uninit.assume_init()) }
// }

// fn write_object<T, W>(writer: &mut W, obj: T) -> Result<(), WriteError>
// where
//     T: Pod,
//     W: Write,
// {
//     let mut uninit: MaybeUninit<T> = MaybeUninit::new(obj);
//     let buf =
//         unsafe { std::slice::from_raw_parts(uninit.as_ptr() as *const u8, mem::size_of::<T>()) };

//     writer.write_all(buf)?;

//     Ok(())
// }

fn write_object<T, W>(writer: &mut W, obj: T) -> Result<(), WriteError>
where
    T: bytemuck::Pod,
    W: Write,
{
    let buf = bytemuck::bytes_of(&obj);

    writer.write_all(buf)?;

    Ok(())
}

pub struct BlockWriter<W: Write + Seek> {
    writer: BufWriter<W>,
}

impl<W> BlockWriter<W>
where
    W: Write + Seek,
{
    pub fn new(handle: W) -> Self {
        Self {
            writer: BufWriter::new(handle),
        }
    }

    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    }

    pub fn write_superblock(&mut self, super_block: SuperBlock) -> Result<(), WriteError> {
        self.writer
            .seek(io::SeekFrom::Start(ROOT_BLOCK_OFFSET.into()))?;

        write_object(&mut self.writer, super_block)?;

        Ok(())
    }
}
