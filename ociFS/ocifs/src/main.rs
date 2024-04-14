// The MIT License (MIT)

// Copyright (c) 2024 AnonymousDapper

#![deny(rust_2018_idioms)]

use clap::{arg, command, value_parser};
use type_layout::TypeLayout;

use colored::Colorize;

fn print_fatal<S: std::fmt::Display>(msg: S) -> ! {
    eprintln!("[{}]: {}", "ocifs error".red().bold(), msg);

    std::process::exit(1);
}

fn open_file<P: AsRef<std::path::Path>>(path_ref: P) -> std::fs::File {
    let path = path_ref.as_ref();

    let f = std::fs::File::options().read(true).write(true).open(path);

    match f {
        Ok(handle) => handle,
        Err(e) => print_fatal(e),
    }
}

fn main() {
    let matches = command!()
        .arg(
            arg!(<file> "Path to block device or image file")
                .value_parser(value_parser!(std::path::PathBuf)),
        )
        .get_matches();

    //let path: &std::path::PathBuf = matches.get_one("file").unwrap();

    //let file = open_file(path);

    // println!("{:#?}", file.metadata().unwrap());

    // println!("{}", ocifs::UUID::type_layout());
    // println!("{}", ocifs::Version::type_layout());
    println!("{}", ocifs::SuperBlock::type_layout());
    println!("{}", ocifs::GroupDescriptor::type_layout());

    /*
    let mut reader = ocifs::read::BlockReader::new(&file);
    let try_super_block = reader.read_superblock();

    let mut super_block = match try_super_block {
        Err(e) => match e {
            ocifs::read::ReadError::BadMagic => print_fatal("Superblock signature is invalid"),
            ocifs::read::ReadError::Invalid => print_fatal("Superblock is corrupt"),
            _ => print_fatal("Unknown error"),
        },
        Ok(b) => b,
    };

    println!("Super block valid");
    println!(
        "Total volume size: {:0.02} MiB",
        reader.get_vol_size().unwrap_or(0) as f32 / 1024f32 / 1024f32
    );
    println!(
        "ociFS File System v{} on {}: '{}' - {}",
        super_block.version,
        path.display(),
        super_block.volume_name().unwrap_or("bad string decode"),
        super_block.fs_id
    );

    super_block.rename("Æ Memetics Division");

    let mut writer = ocifs::write::BlockWriter::new(&file);
    writer.write_superblock(super_block).unwrap();
    writer.flush();
    file.sync_all().unwrap();

    println!("Renamed volume");
    let new_block = reader.read_superblock().unwrap();
    println!(
        "ociFS File System v{} on {}: '{}' - {}",
        new_block.version,
        path.display(),
        new_block.volume_name().unwrap_or("bad string decode"),
        new_block.fs_id
    );*/
}
